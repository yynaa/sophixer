use crate::devices::launchpad_mini_mk3::input::LPM3InputMessage;
use crate::devices::launchpad_mini_mk3::output::LPM3OutputMessage;
use crate::devices::launchpad_mini_mk3::visual::LPM3Visual;
use crate::devices::launchpad_mini_mk3::LPM3Position;
use crate::devices::{get_in_port, get_out_port};
use crate::{
  MidiDriver, MidiDriverError, MidiInputMessage, MidiOutputMessage, MidiPhysicalPosition,
  MidiPhysicalState, MidiVisual,
};
use midir::{MidiInputConnection, MidiOutputConnection};
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;

pub struct LPM3Driver {
  _conn_in: MidiInputConnection<()>,
  conn_out: MidiOutputConnection,
  rx: mpsc::Receiver<LPM3InputMessage>,

  effective_visual: HashMap<u8, LPM3Visual>,
  queued_visual_changes: HashMap<u8, LPM3Visual>,

  physical_states: HashMap<u8, MidiPhysicalState>,
}

impl MidiDriver<LPM3InputMessage, LPM3OutputMessage, LPM3Visual, LPM3Position> for LPM3Driver {
  fn connect() -> Result<Self, MidiDriverError> {
    debug!("LPM3 -- starting driver...");

    let (midi_in, in_port) = get_in_port("Launchpad Mini MK3")?;
    let (midi_out, out_port) = get_out_port("Launchpad Mini MK3")?;

    let conn_out = midi_out
      .connect(&out_port, "LPMiniMK3 MIDI output writer")
      .map_err(|e| MidiDriverError::MidirConnectOutputError(e.to_string()))?;

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
      .map_err(|e| MidiDriverError::MidirConnectInputError(e.to_string()))?;

    let mut physical_states = HashMap::new();
    for y in 1..=9 {
      for x in 1..=9 {
        if x != y && y != 9 {
          physical_states.insert(y * 10 + x, MidiPhysicalState::Binary(false));
        }
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
    debug!("LPM3 -- closing driver...");

    self.clear()?;
    self.push()?;

    Ok(())
  }

  fn read(&mut self) -> Result<VecDeque<LPM3InputMessage>, MidiDriverError> {
    let mut q = VecDeque::new();

    loop {
      match self.rx.try_recv() {
        Ok(msg) => {
          match &msg {
            LPM3InputMessage::KeyPressed(pos) => self
              .physical_states
              .insert(pos.to_raw()?, MidiPhysicalState::Binary(true)),
            LPM3InputMessage::KeyReleased(pos) => self
              .physical_states
              .insert(pos.to_raw()?, MidiPhysicalState::Binary(false)),
          };
          q.push_back(msg);
        }
        Err(_) => break,
      }
    }

    Ok(q)
  }

  fn get_position_state(&self, pos: LPM3Position) -> Result<MidiPhysicalState, MidiDriverError> {
    let raw = pos.to_raw()?;
    self
      .physical_states
      .get(&raw)
      .ok_or(MidiDriverError::InvalidPosition(
        "Launch Control XL MK2".to_string(),
        format!("{} as a button", raw),
      ))
      .map(|t| t.clone())
  }

  fn send(&mut self, msg: LPM3OutputMessage) -> Result<(), MidiDriverError> {
    trace!("LPM3 -- sending message: {:?}", msg);
    self
      .conn_out
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
          self
            .effective_visual
            .insert(*change_pos, change_visual.clone());
        }
      } else {
        v.push(change_visual.clone());
        self
          .effective_visual
          .insert(*change_pos, change_visual.clone());
      }
    }

    if v.len() > 0 {
      self.send(LPM3OutputMessage::SendColors(v))?;
      self.pop();
    }
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
