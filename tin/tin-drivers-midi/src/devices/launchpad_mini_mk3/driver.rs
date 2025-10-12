use crate::devices::launchpad_mini_mk3::input::LPM3InputMessage;
use crate::devices::launchpad_mini_mk3::output::LPM3OutputMessage;
use crate::devices::launchpad_mini_mk3::visual::LPM3Visual;
use crate::devices::launchpad_mini_mk3::LPM3Position;
use crate::devices::{get_in_port, get_out_port};
use crate::{MidiDriver, MidiDriverError, MidiInputMessage, MidiOutputMessage, MidiVisual};
use midir::{MidiInputConnection, MidiOutputConnection};
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;

pub struct LPM3Driver {
    _conn_in: MidiInputConnection<()>,
    conn_out: MidiOutputConnection,
    rx: mpsc::Receiver<LPM3InputMessage>,

    effective_visual: HashMap<u8, LPM3Visual>,
    queued_visual_changes: HashMap<u8, LPM3Visual>,
}

impl MidiDriver<LPM3InputMessage, LPM3OutputMessage, LPM3Visual> for LPM3Driver {
    fn connect() -> Result<Self, MidiDriverError> {
        let (midi_in, in_port) = get_in_port("(LPMiniMK3 MIDI)")?;
        let (midi_out, out_port) = get_out_port("(LPMiniMK3 MIDI)")?;

        let conn_out = midi_out
            .connect(&out_port, "LPMiniMK3 MIDI output writer")
            .map_err(MidiDriverError::MidirConnectOutputError)?;

        let (tx, rx) = mpsc::channel::<LPM3InputMessage>();

        let _conn_in = midi_in
            .connect(
                &in_port,
                "input-reader",
                move |_, raw_message, _| {
                    if let Some(message) = LPM3InputMessage::from_raw(raw_message) {
                        let _ = tx.send(message);
                    }
                },
                (),
            )
            .map_err(MidiDriverError::MidirConnectInputError)?;

        let mut s = Self {
            rx,
            _conn_in,
            conn_out,
            effective_visual: HashMap::new(),
            queued_visual_changes: HashMap::new(),
        };

        // turning off DAW mode
        s.send(LPM3OutputMessage::Raw(vec![
            240, 0, 32, 41, 2, 13, 16, 0, 247,
        ]))?;

        // clear
        s.clear()?;
        s.push()?;

        Ok(s)
    }

    fn close(&mut self) -> Result<(), MidiDriverError> {
        self.clear()?;
        self.push()?;

        Ok(())
    }

    fn read(&mut self) -> Result<VecDeque<LPM3InputMessage>, MidiDriverError> {
        let mut q = VecDeque::new();

        loop {
            match self.rx.try_recv() {
                Ok(msg) => q.push_back(msg),
                Err(_) => break,
            }
        }

        Ok(q)
    }

    fn send(&mut self, msg: LPM3OutputMessage) -> Result<(), MidiDriverError> {
        self.conn_out
            .send(&msg.to_raw()?)
            .map_err(MidiDriverError::MidirSendError)
    }

    fn pop(&mut self) {
        self.queued_visual_changes.clear();
    }

    fn push(&mut self) -> Result<(), MidiDriverError> {
        let mut v: Vec<LPM3Visual> = Vec::new();

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

        self.send(LPM3OutputMessage::SendColors(v))?;
        self.pop();
        Ok(())
    }

    fn clear(&mut self) -> Result<(), MidiDriverError> {
        self.pop();
        for y in 1..=9 {
            for x in 1..=9 {
                self.add(LPM3Visual::Off(LPM3Position::Raw(y * 10 + x)))?;
            }
        }
        Ok(())
    }

    fn add(&mut self, visual: LPM3Visual) -> Result<(), MidiDriverError> {
        let p = visual.get_raw_pos()?;
        self.queued_visual_changes.insert(p, visual);
        Ok(())
    }
}
