use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{
  data::{
    buttons::{ActionDescriptor, SongButtonActionValue},
    channels::Channel,
  },
  messages::renoise::to::MuteTrackSequenceSlot,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct ToggleTrackPatterns {
  pub track_patterns: HashSet<(Channel, u16)>,
  pub default: bool,
  pub color_off: [u8; 3],
  pub color_on: [u8; 3],
}

impl ToggleTrackPatterns {
  fn color_from_bool(&self, b: bool) -> [u8; 3] {
    if b { self.color_on } else { self.color_off }
  }
}

impl ActionDescriptor for ToggleTrackPatterns {
  fn get_default(&self) -> SongButtonActionValue {
    SongButtonActionValue::Boolean(self.default)
  }

  fn get_default_color(&self) -> [u8; 3] {
    self.color_from_bool(self.default)
  }

  fn get_color(&self, value: SongButtonActionValue) -> Result<[u8; 3]> {
    match value {
      SongButtonActionValue::Boolean(b) => Ok(self.color_from_bool(b)),
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }

  fn next(&self, value: SongButtonActionValue) -> Result<SongButtonActionValue> {
    match value {
      SongButtonActionValue::Boolean(b) => Ok(SongButtonActionValue::Boolean(!b)),
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }

  fn create_renoise_message(
    &self,
    value: SongButtonActionValue,
  ) -> Result<Vec<crate::messages::renoise::to::MessageToRenoise>> {
    match value {
      SongButtonActionValue::Boolean(b) => {
        let mut msgs = Vec::new();
        for (c, seq) in &self.track_patterns {
          msgs.push(
            MuteTrackSequenceSlot {
              track: c.to_renoise_number(),
              sequence: *seq,
              bypass: !b,
            }
            .into(),
          );
        }
        Ok(msgs)
      }
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }
}
