use std::{collections::VecDeque, net::SocketAddr, time::Duration};

use crate::model::{RenoiseInstance, TinModel};
use anyhow::Result;
use tin_drivers_midi::{
  devices::{
    launch_control_xl_mk2::{LCXL2Driver, LCXL2InputMessage, LCXL2Position, LCXL2Visual},
    launchpad_mini_mk3::{LPM3Driver, LPM3Position, LPM3Visual},
  },
  MidiDriver,
};

pub struct ViewLCXL2Panel {}

impl ViewLCXL2Panel {
  pub fn new() -> Self {
    Self {}
  }

  pub fn update(
    &mut self,
    dt: &Duration,
    tin: &mut TinModel,
    lcxl2: &mut LCXL2Driver,
    lcxl2_inputs: VecDeque<LCXL2InputMessage>,
  ) -> Result<()> {
    for i in lcxl2_inputs {
      if i == LCXL2InputMessage::KeyPressed(LCXL2Position::Left) {
        let mut sorted = if let Some(risa) = &tin.renoise_instance_focus {
          let ri_id = tin
            .renoise_instance_ids
            .get_by_right(risa)
            .ok_or(anyhow::Error::msg("couldn't find renoise instance id"))?;
          tin
            .renoise_instances
            .iter()
            .filter(|f| *tin.renoise_instance_ids.get_by_right(f.0).unwrap_or(&0u64) < *ri_id)
            .collect::<Vec<(&SocketAddr, &RenoiseInstance)>>()
        } else {
          tin
            .renoise_instances
            .iter()
            .collect::<Vec<(&SocketAddr, &RenoiseInstance)>>()
        };
        sorted.sort_by_key(|f| *tin.renoise_instance_ids.get_by_right(f.0).unwrap_or(&0u64));
        tin.renoise_instance_focus = sorted.last().map(|f| f.0.clone());
      }
      if i == LCXL2InputMessage::KeyPressed(LCXL2Position::Right) {
        let mut sorted = if let Some(risa) = &tin.renoise_instance_focus {
          let ri_id = tin
            .renoise_instance_ids
            .get_by_right(risa)
            .ok_or(anyhow::Error::msg("couldn't find renoise instance id"))?;
          tin
            .renoise_instances
            .iter()
            .filter(|f| *tin.renoise_instance_ids.get_by_right(f.0).unwrap_or(&0u64) > *ri_id)
            .collect::<Vec<(&SocketAddr, &RenoiseInstance)>>()
        } else {
          tin
            .renoise_instances
            .iter()
            .collect::<Vec<(&SocketAddr, &RenoiseInstance)>>()
        };
        sorted.sort_by_key(|f| *tin.renoise_instance_ids.get_by_right(f.0).unwrap_or(&0u64));
        tin.renoise_instance_focus = sorted.first().map(|f| f.0.clone());
      }
      debug!("switched to: {:?}", tin.renoise_instance_focus);
    }

    Ok(())
  }

  pub fn draw(&self, tin: &TinModel, lcxl2: &mut LCXL2Driver) -> Result<()> {
    lcxl2.add(LCXL2Visual::Static(LCXL2Position::Left, 3, 0))?;
    lcxl2.add(LCXL2Visual::Static(LCXL2Position::Right, 3, 0))?;

    Ok(())
  }
}
