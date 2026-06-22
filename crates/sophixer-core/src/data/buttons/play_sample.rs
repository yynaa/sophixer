use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::data::{
  buttons::{ActionDescriptor, SongButtonActionValue},
  channels::Channel,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct PlaySample {
  pub track: Channel,
  pub pitch: u8,
  pub volume: u8,
  pub sample: u64,
  pub color: [u8; 3],
}

impl ActionDescriptor for PlaySample {
  fn get_default(&self) -> SongButtonActionValue {
    SongButtonActionValue::None
  }

  fn get_default_color(&self) -> [u8; 3] {
    self.color
  }

  fn get_color(&self, value: SongButtonActionValue) -> Result<[u8; 3]> {
    match value {
      SongButtonActionValue::None => Ok(self.color),
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }

  fn next(&self, value: SongButtonActionValue) -> Result<SongButtonActionValue> {
    match value {
      SongButtonActionValue::None => Ok(SongButtonActionValue::None),
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }

  fn create_renoise_message(
    &self,
    value: SongButtonActionValue,
  ) -> Result<Vec<crate::messages::renoise::MessageToRenoise>> {
    match value {
      SongButtonActionValue::None => {
        todo!()
      }
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }
}
