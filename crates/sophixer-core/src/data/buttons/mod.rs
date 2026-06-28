use std::fmt::Display;

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::messages::renoise::to::MessageToRenoise;

pub mod cycle_effect_parameter_value;
pub mod play_sample;
pub mod toggle_channels;
pub mod toggle_effect_bypass;
pub mod toggle_track_patterns;

#[enum_dispatch]
pub trait ActionDescriptor {
  fn get_default(&self) -> SongButtonActionValue;
  fn get_default_color(&self) -> [u8; 3];

  fn get_color(&self, value: SongButtonActionValue) -> Result<[u8; 3]>;

  fn next(&self, value: SongButtonActionValue) -> Result<SongButtonActionValue>;

  fn create_renoise_message(&self, value: SongButtonActionValue) -> Result<Vec<MessageToRenoise>>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[enum_dispatch(ActionDescriptor)]
pub enum SongButtonAction {
  ToggleChannels(toggle_channels::ToggleChannels),
  ToggleTrackPatterns(toggle_track_patterns::ToggleTrackPatterns),
  ToggleEffectBypass(toggle_effect_bypass::ToggleEffectBypass),
  CycleEffectParameterValue(cycle_effect_parameter_value::CycleEffectParameterValue),
  PlaySample(play_sample::PlaySample),
}

impl Display for SongButtonAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ToggleChannels(_) => write!(f, "ToggleChannels"),
      Self::ToggleTrackPatterns(_) => write!(f, "ToggleTrackPatterns"),
      Self::ToggleEffectBypass(_) => write!(f, "ToggleEffectBypass"),
      Self::CycleEffectParameterValue(_) => write!(f, "CycleEffectParameterValue"),
      Self::PlaySample(_) => write!(f, "PlaySample"),
    }
  }
}

impl Default for SongButtonAction {
  fn default() -> Self {
    Self::PlaySample(play_sample::PlaySample::default())
  }
}

#[derive(Clone, Copy)]
pub enum SongButtonActionValue {
  None,
  Boolean(bool),
  Number(usize),
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
