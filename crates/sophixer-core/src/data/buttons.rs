use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    channels: HashSet<u64>,
    default: bool,
    color_off: [u8; 3],
    color_on: [u8; 3],
  },
  ToggleTrackPatterns {
    track_patterns: HashSet<(u64, u64)>,
    default: bool,
    color_off: [u8; 3],
    color_on: [u8; 3],
  },
  ToggleEffectBypass {
    track: u64,
    effect: u64,
    default: bool,
    color_off: [u8; 3],
    color_on: [u8; 3],
  },
  CycleEffectParameterValue {
    track: u64,
    effect: u64,
    param: u64,
    default: usize,
    cycles: Vec<CycleEffectParameterValue>,
  },
  PlaySample {
    track: u64,
    pitch: u8,
    volume: u8,
    sample: u64,
    color: [u8; 3],
  },
}

pub enum SongButtonActionDefault {
  None,
  Boolean(bool),
  Number(usize),
}

impl SongButtonAction {
  pub fn get_default(&self) -> SongButtonActionDefault {
    match self {
      Self::PlaySample {
        track: _,
        pitch: _,
        volume: _,
        sample: _,
        color: _,
      } => SongButtonActionDefault::None,
      Self::ToggleChannels {
        channels: _,
        default,
        color_off: _,
        color_on: _,
      } => SongButtonActionDefault::Boolean(*default),
      Self::CycleEffectParameterValue {
        track: _,
        effect: _,
        param: _,
        default,
        cycles: _,
      } => SongButtonActionDefault::Number(*default),
      Self::ToggleEffectBypass {
        track: _,
        effect: _,
        default,
        color_off: _,
        color_on: _,
      } => SongButtonActionDefault::Boolean(*default),
      Self::ToggleTrackPatterns {
        track_patterns: _,
        default,
        color_off: _,
        color_on: _,
      } => SongButtonActionDefault::Boolean(*default),
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
      track: 1,
      effect: 1,
      default: true,
      color_off: [255, 0, 0],
      color_on: [0, 0, 255],
    })
  }

  pub fn default_cycle_effect_parameter_value() -> Result<SongButtonAction> {
    Ok(SongButtonAction::CycleEffectParameterValue {
      track: 1,
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
      track: 0,
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
