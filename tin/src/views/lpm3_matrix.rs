use crate::model::TinModel;
use anyhow::Result;
use tin_drivers_midi::devices::launchpad_mini_mk3::LPM3Driver;

pub struct ViewLPM3Matrix {}

impl ViewLPM3Matrix {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, tin: &mut TinModel, lpm3: &mut LPM3Driver) -> Result<()> {
        Ok(())
    }

    pub fn draw(&self, tin: &TinModel, lpm3: &mut LPM3Driver) -> Result<()> {
        Ok(())
    }
}
