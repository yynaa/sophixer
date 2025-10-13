use crate::devices::launch_control_xl_mk2::input::LCXL2InputMessage;
use crate::devices::launch_control_xl_mk2::output::LCXL2OutputMessage;
use crate::devices::launch_control_xl_mk2::visual::LCXL2Visual;
use crate::devices::launch_control_xl_mk2::LCXL2Position;
use crate::devices::{get_in_port, get_out_port};
use crate::{
    MidiDriver, MidiDriverError, MidiInputMessage, MidiOutputMessage, MidiPhysicalPosition,
    MidiPhysicalState, MidiVisual,
};
use midir::{MidiInputConnection, MidiOutputConnection};
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;

pub struct LCXL2Driver {
    _conn_in: MidiInputConnection<()>,
    conn_out: MidiOutputConnection,
    rx: mpsc::Receiver<LCXL2InputMessage>,

    effective_visual: HashMap<u8, LCXL2Visual>,
    queued_visual_changes: HashMap<u8, LCXL2Visual>,

    physical_states: HashMap<u8, MidiPhysicalState>,
}

impl MidiDriver<LCXL2InputMessage, LCXL2OutputMessage, LCXL2Visual, LCXL2Position> for LCXL2Driver {
    fn connect() -> Result<Self, MidiDriverError> {
        debug!("LCXL2 -- starting driver...");

        let (midi_in, in_port) = get_in_port("2- Launch Control XL")?;
        let (midi_out, out_port) = get_out_port("2- Launch Control XL")?;

        let conn_out = midi_out
            .connect(&out_port, "LCXL2 MIDI output writer")
            .map_err(MidiDriverError::MidirConnectOutputError)?;

        let (tx, rx) = mpsc::channel::<LCXL2InputMessage>();

        let _conn_in = midi_in
            .connect(
                &in_port,
                "input-reader",
                move |_, raw_message, _| {
                    if let Some(message) = LCXL2InputMessage::from_raw(raw_message) {
                        let _ = tx.send(message);
                    }
                },
                (),
            )
            .map_err(MidiDriverError::MidirConnectInputError)?;

        let mut physical_states = HashMap::new();
        for r in 0..56 {
            if LCXL2Position::is_analog_raw(&r) {
                physical_states.insert(r, MidiPhysicalState::Analog8(0));
            } else {
                physical_states.insert(r, MidiPhysicalState::Binary(false));
            }
        }

        let mut s = Self {
            rx,
            _conn_in,
            conn_out,
            effective_visual: HashMap::new(),
            queued_visual_changes: HashMap::new(),
            physical_states,
        };

        // clear
        s.clear()?;
        s.push()?;

        Ok(s)
    }

    fn close(&mut self) -> Result<(), MidiDriverError> {
        debug!("LCXL2 -- closing driver...");

        self.clear()?;
        self.push()?;

        Ok(())
    }

    fn read(&mut self) -> Result<VecDeque<LCXL2InputMessage>, MidiDriverError> {
        let mut q = VecDeque::new();

        let mut analog_messages = HashMap::new();

        loop {
            match self.rx.try_recv() {
                Ok(msg) => match &msg {
                    LCXL2InputMessage::Analog(pos, value) => {
                        analog_messages.insert(
                            pos.to_raw()?,
                            LCXL2InputMessage::Analog(pos.clone(), *value),
                        );
                    }
                    LCXL2InputMessage::KeyPressed(pos) => {
                        self.physical_states
                            .insert(pos.to_raw()?, MidiPhysicalState::Binary(true));
                        q.push_back(msg);
                    }
                    LCXL2InputMessage::KeyReleased(pos) => {
                        self.physical_states
                            .insert(pos.to_raw()?, MidiPhysicalState::Binary(false));
                        q.push_back(msg);
                    }
                },
                Err(_) => break,
            }
        }

        for (k, v) in analog_messages {
            let LCXL2InputMessage::Analog(_, value) = v else {
                unreachable!("this will never not be Analog")
            };
            self.physical_states
                .insert(k, MidiPhysicalState::Analog8(value));
            q.push_back(v);
        }

        Ok(q)
    }

    fn get_position_state(&self, pos: LCXL2Position) -> Result<MidiPhysicalState, MidiDriverError> {
        let raw = pos.to_raw()?;
        self.physical_states
            .get(&raw)
            .ok_or(MidiDriverError::InvalidPosition(
                "Launch Control XL MK2".to_string(),
                format!("{} as a button", raw),
            ))
            .map(|t| t.clone())
    }

    fn send(&mut self, msg: LCXL2OutputMessage) -> Result<(), MidiDriverError> {
        trace!("LCXL2 -- sending message: {:?}", msg);
        self.conn_out
            .send(&msg.to_raw()?)
            .map_err(MidiDriverError::MidirSendError)
    }

    fn pop(&mut self) {
        self.queued_visual_changes.clear();
    }

    fn push(&mut self) -> Result<(), MidiDriverError> {
        let mut v: Vec<LCXL2Visual> = Vec::new();

        for (change_pos, change_visual) in self.queued_visual_changes.iter() {
            if let Some(current_visual) = self.effective_visual.get(&change_pos) {
                if !change_visual.is_absorbed(current_visual) {
                    v.push(change_visual.clone());
                    self.effective_visual
                        .insert(*change_pos, change_visual.clone());
                }
            } else {
                v.push(change_visual.clone());
                self.effective_visual
                    .insert(*change_pos, change_visual.clone());
            }
        }

        if v.len() > 0 {
            self.send(LCXL2OutputMessage::SendColors(v))?;
            self.pop();
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<(), MidiDriverError> {
        self.pop();
        for r in 1..48 {
            self.add(LCXL2Visual::Off(LCXL2Position::Raw(r)))?;
        }
        Ok(())
    }

    fn add(&mut self, visual: LCXL2Visual) -> Result<(), MidiDriverError> {
        let p = visual.get_raw_pos()?;
        self.queued_visual_changes.insert(p, visual);
        Ok(())
    }
}
