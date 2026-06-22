use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{data::channels::Channel, messages::renoise::MessageToRenoise};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CycleEffectParameterValue {
  pub value: f64,
  pub color: [u8; 3],
}

impl Default for CycleEffectParameterValue {
  fn default() -> Self {
    Self {
      value: 0.5,
      color: [255, 255, 255],
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SongButtonAction {
  ToggleChannels {
    channels: HashSet<Channel>,
    default: bool,
    color_off: [u8; 3],
    color_on: [u8; 3],
  },
  ToggleTrackPatterns {
    track_patterns: HashSet<(Channel, u64)>,
    default: bool,
    color_off: [u8; 3],
    color_on: [u8; 3],
  },
  ToggleEffectBypass {
    track: Channel,
    effect: u64,
    default: bool,
    color_off: [u8; 3],
    color_on: [u8; 3],
  },
  CycleEffectParameterValue {
    track: Channel,
    effect: u64,
    param: u64,
    default: usize,
    cycles: Vec<CycleEffectParameterValue>,
  },
  PlaySample {
    track: Channel,
    pitch: u8,
    volume: u8,
    sample: u64,
    color: [u8; 3],
  },
}

#[derive(Clone, Copy)]
pub enum SongButtonActionValue {
  None,
  Boolean(bool),
  Number(usize),
}

impl SongButtonAction {
  pub fn get_default(&self) -> SongButtonActionValue {
    match self {
      Self::PlaySample {
        track: _,
        pitch: _,
        volume: _,
        sample: _,
        color: _,
      } => SongButtonActionValue::None,
      Self::ToggleChannels {
        channels: _,
        default,
        color_off: _,
        color_on: _,
      } => SongButtonActionValue::Boolean(*default),
      Self::CycleEffectParameterValue {
        track: _,
        effect: _,
        param: _,
        default,
        cycles: _,
      } => SongButtonActionValue::Number(*default),
      Self::ToggleEffectBypass {
        track: _,
        effect: _,
        default,
        color_off: _,
        color_on: _,
      } => SongButtonActionValue::Boolean(*default),
      Self::ToggleTrackPatterns {
        track_patterns: _,
        default,
        color_off: _,
        color_on: _,
      } => SongButtonActionValue::Boolean(*default),
    }
  }
  pub fn get_default_color(&self) -> [u8; 3] {
    match self {
      Self::CycleEffectParameterValue {
        track: _,
        effect: _,
        param: _,
        default,
        cycles,
      } => cycles.get(*default).map(|f| f.color).unwrap_or([0, 0, 0]),
      Self::PlaySample {
        track: _,
        pitch: _,
        volume: _,
        sample: _,
        color,
      } => *color,
      Self::ToggleChannels {
        channels: _,
        default,
        color_off,
        color_on,
      } => {
        if *default {
          *color_on
        } else {
          *color_off
        }
      }
      Self::ToggleEffectBypass {
        track: _,
        effect: _,
        default,
        color_off,
        color_on,
      } => {
        if *default {
          *color_on
        } else {
          *color_off
        }
      }
      Self::ToggleTrackPatterns {
        track_patterns: _,
        default,
        color_off,
        color_on,
      } => {
        if *default {
          *color_on
        } else {
          *color_off
        }
      }
    }
  }

  pub fn get_color(&self, value: SongButtonActionValue) -> Result<[u8; 3]> {
    match (value, self) {
      (
        SongButtonActionValue::None,
        Self::PlaySample {
          track: _,
          pitch: _,
          volume: _,
          sample: _,
          color,
        },
      ) => Ok(*color),
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleChannels {
          channels: _,
          default: _,
          color_off,
          color_on,
        },
      ) => Ok(match b {
        true => *color_on,
        false => *color_off,
      }),
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleEffectBypass {
          track: _,
          effect: _,
          default: _,
          color_off,
          color_on,
        },
      ) => Ok(match b {
        true => *color_on,
        false => *color_off,
      }),
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleTrackPatterns {
          track_patterns: _,
          default: _,
          color_off,
          color_on,
        },
      ) => Ok(match b {
        true => *color_on,
        false => *color_off,
      }),
      (
        SongButtonActionValue::Number(n),
        Self::CycleEffectParameterValue {
          track: _,
          effect: _,
          param: _,
          default: _,
          cycles,
        },
      ) => cycles
        .get(n)
        .map(|f| f.color)
        .ok_or(anyhow::Error::msg("cycle not found")),
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }
  pub fn next(&self, value: SongButtonActionValue) -> Result<SongButtonActionValue> {
    match (value, self) {
      (
        SongButtonActionValue::None,
        Self::PlaySample {
          track: _,
          pitch: _,
          volume: _,
          sample: _,
          color: _,
        },
      ) => Ok(SongButtonActionValue::None),
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleChannels {
          channels: _,
          default: _,
          color_off: _,
          color_on: _,
        },
      ) => Ok(SongButtonActionValue::Boolean(!b)),
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleEffectBypass {
          track: _,
          effect: _,
          default: _,
          color_off: _,
          color_on: _,
        },
      ) => Ok(SongButtonActionValue::Boolean(!b)),
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleTrackPatterns {
          track_patterns: _,
          default: _,
          color_off: _,
          color_on: _,
        },
      ) => Ok(SongButtonActionValue::Boolean(!b)),
      (
        SongButtonActionValue::Number(n),
        Self::CycleEffectParameterValue {
          track: _,
          effect: _,
          param: _,
          default: _,
          cycles,
        },
      ) => {
        let len = cycles.len();
        let mut new = n + 1;
        if new >= len {
          new = 0;
        }
        Ok(SongButtonActionValue::Number(new))
      }
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }
  pub fn create_renoise_message(
    &self,
    value: SongButtonActionValue,
  ) -> Result<Vec<MessageToRenoise>> {
    match (value, self) {
      #[allow(unused)]
      (
        SongButtonActionValue::None,
        Self::PlaySample {
          track,
          pitch,
          volume,
          sample,
          color: _,
        },
      ) => Ok(todo!()),
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleChannels {
          channels,
          default: _,
          color_off: _,
          color_on: _,
        },
      ) => {
        let mut msgs = Vec::new();
        for c in channels {
          msgs.push(MessageToRenoise::MuteTrack(c.to_renoise_number(), !b));
        }
        Ok(msgs)
      }
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleEffectBypass {
          track,
          effect,
          default: _,
          color_off: _,
          color_on: _,
        },
      ) => Ok(vec![MessageToRenoise::BypassEffect(
        track.to_renoise_number(),
        *effect,
        b,
      )]),
      (
        SongButtonActionValue::Boolean(b),
        Self::ToggleTrackPatterns {
          track_patterns,
          default: _,
          color_off: _,
          color_on: _,
        },
      ) => {
        let mut msgs = Vec::new();
        for (c, seq) in track_patterns {
          msgs.push(MessageToRenoise::MuteTrackSequenceSlot(
            c.to_renoise_number(),
            *seq,
            b,
          ));
        }
        Ok(msgs)
      }
      (
        SongButtonActionValue::Number(n),
        Self::CycleEffectParameterValue {
          track,
          effect,
          param,
          default: _,
          cycles,
        },
      ) => {
        let c = cycles.get(n).ok_or(anyhow::Error::msg("no such cycle"))?;
        Ok(vec![MessageToRenoise::SetParameterValue(
          track.to_renoise_number(),
          *effect,
          *param,
          c.value,
        )])
      }
      _ => Err(anyhow::Error::msg("invalid value")),
    }
  }

  pub fn default_toggle_channels() -> Result<SongButtonAction> {
    Ok(SongButtonAction::ToggleChannels {
      channels: HashSet::new(),
      default: true,
      color_off: [255, 0, 0],
      color_on: [0, 0, 255],
    })
  }

  pub fn default_toggle_track_patterns() -> Result<SongButtonAction> {
    Ok(SongButtonAction::ToggleTrackPatterns {
      track_patterns: HashSet::new(),
      default: true,
      color_off: [255, 0, 0],
      color_on: [0, 0, 255],
    })
  }

  pub fn default_toggle_effect_bypass() -> Result<SongButtonAction> {
    Ok(SongButtonAction::ToggleEffectBypass {
      track: Channel::Master,
      effect: 1,
      default: true,
      color_off: [255, 0, 0],
      color_on: [0, 0, 255],
    })
  }

  pub fn default_cycle_effect_parameter_value() -> Result<SongButtonAction> {
    Ok(SongButtonAction::CycleEffectParameterValue {
      track: Channel::Master,
      effect: 1,
      param: 1,
      default: 0,
      cycles: Vec::from([CycleEffectParameterValue {
        value: 0.,
        color: [0, 255, 0],
      }]),
    })
  }

  pub fn default_play_sample() -> Result<SongButtonAction> {
    Ok(SongButtonAction::PlaySample {
      track: Channel::Master,
      pitch: 48,
      volume: 255,
      sample: 0,
      color: [255, 255, 255],
    })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SongButton {
  pub action: SongButtonAction,
}

impl SongButton {
  pub fn new(action: SongButtonAction) -> Result<SongButton> {
    Ok(Self { action })
  }
}
