use std::time::Duration;

use crate::model::TinModel;
use anyhow::Result;
use tin_drivers_midi::{
  devices::launchpad_mini_mk3::{LPM3Driver, LPM3Position, LPM3Visual},
  MidiDriver,
};

pub struct ViewLPM3Matrix {}

impl ViewLPM3Matrix {
  pub fn new() -> Self {
    Self {}
  }

  pub fn update(&mut self, dt: &Duration, tin: &mut TinModel, lpm3: &mut LPM3Driver) -> Result<()> {
    Ok(())
  }

  pub fn draw(&self, tin: &TinModel, lpm3: &mut LPM3Driver) -> Result<()> {
    { // CONTROL PANEL
    }

    Ok(())
  }
}
