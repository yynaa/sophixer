use crate::devices::launchpad_mini_mk3::LPM3Position;
use crate::MidiInputMessage;

#[derive(Debug, Clone)]
pub enum LPM3InputMessage {
  KeyPressed(LPM3Position),
  KeyReleased(LPM3Position),
}

impl MidiInputMessage for LPM3InputMessage {
  fn from_raw(raw: &[u8]) -> Option<Self> {
    if raw.len() == 3 {
      let ty = raw[0];
      if ty == 176 || ty == 144 {
        let pos = raw[1];
        let value = raw[2];
        if value > 0 {
          Some(LPM3InputMessage::KeyPressed(LPM3Position::Raw(pos)))
        } else {
          Some(LPM3InputMessage::KeyReleased(LPM3Position::Raw(pos)))
        }
      } else {
        None
      }
    } else {
      None
    }
  }
}

impl PartialEq for LPM3InputMessage {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Self::KeyPressed(a) => match other {
        Self::KeyPressed(b) => a == b,
        _ => false,
      },
      Self::KeyReleased(a) => match other {
        Self::KeyReleased(b) => a == b,
        _ => false,
      },
    }
  }
}

impl Eq for LPM3InputMessage {}
