use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
  data::{
    buttons::{ActionDescriptor, SongButtonActionValue},
    channels::Channel,
  },
  messages::renoise::to::SetParameterValue,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParameterValue {
  pub value: f64,
  pub color: [u8; 3],
}

impl Default for ParameterValue {
  fn default() -> Self {
    Self {
      value: 0.5,
      color: [255, 255, 255],
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CycleEffectParameterValue {
  pub track: Channel,
  pub effect: u16,
  pub param: u16,
  pub default: usize,
  pub cycles: Vec<ParameterValue>,
}

impl CycleEffectParameterValue {
  fn color_from_usize(&self, n: usize) -> Result<[u8; 3]> {
    self
      .cycles
      .get(n)
      .map(|f| f.color)
      .ok_or(anyhow::Error::msg("couldn't find color"))
  }
}

impl ActionDescriptor for CycleEffectParameterValue {
  fn get_default(&self) -> SongButtonActionValue {
    SongButtonActionValue::Number(self.default)
  }

  fn get_default_color(&self) -> [u8; 3] {
    // TODO: if there is an error here, it means the set is malformed!
    self.color_from_usize(self.default).unwrap()
  }

  fn get_color(&self, value: SongButtonActionValue) -> Result<[u8; 3]> {
    match value {
      SongButtonActionValue::Number(n) => self.color_from_usize(n),
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }

  fn next(&self, value: SongButtonActionValue) -> Result<SongButtonActionValue> {
    match value {
      SongButtonActionValue::Number(n) => {
        let len = self.cycles.len();
        let mut new = n + 1;
        if new >= len {
          new = 0;
        }
        Ok(SongButtonActionValue::Number(new))
      }
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }

  fn create_renoise_message(
    &self,
    value: SongButtonActionValue,
  ) -> Result<Vec<crate::messages::renoise::to::MessageToRenoise>> {
    match value {
      SongButtonActionValue::Number(n) => {
        let c = self
          .cycles
          .get(n)
          .ok_or(anyhow::Error::msg("no such cycle"))?;
        Ok(vec![
          SetParameterValue {
            track: self.track.to_renoise_number(),
            effect: self.effect,
            parameter: self.param,
            value: c.value,
          }
          .into(),
        ])
      }
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }
}
