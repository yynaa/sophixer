use std::{collections::VecDeque, net::SocketAddr, time::Duration};

use crate::{
  model::{RenoiseInstance, TinModel},
  servers::renoise::RenoiseCommunicator,
};
use anyhow::Result;
use intercom::server::{udp::UdpServer, InterServerCommunicator};
use sophixer_core::messages::renoise::MessageToRenoise;
use tin_drivers_midi::{
  devices::launch_control_xl_mk2::{LCXL2Driver, LCXL2InputMessage, LCXL2Position, LCXL2Visual},
  MidiDriver,
};

pub struct ViewLCXL2Panel {}

impl ViewLCXL2Panel {
  pub fn new() -> Self {
    Self {}
  }

  pub fn update(
    &mut self,
    _dt: &Duration,
    tin: &mut TinModel,
    _lcxl2: &mut LCXL2Driver,
    lcxl2_inputs: VecDeque<LCXL2InputMessage>,
    server: &UdpServer,
  ) -> Result<()> {
    let static_set = tin.set.clone();
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
        trace!("switched focus to {:?}", tin.renoise_instance_focus);
      }

      if let Some(risa_focus) = tin.renoise_instance_focus {
        if i == LCXL2InputMessage::KeyPressed(LCXL2Position::Up) {
          tin.renoise_instance_a = Some(risa_focus);
        }
        if i == LCXL2InputMessage::KeyPressed(LCXL2Position::Down) {
          tin.renoise_instance_b = Some(risa_focus);
        }
      } else {
        if i == LCXL2InputMessage::KeyPressed(LCXL2Position::Up) {
          tin.renoise_instance_a = None;
        }
        if i == LCXL2InputMessage::KeyPressed(LCXL2Position::Down) {
          tin.renoise_instance_b = None;
        }
      }

      if let Some((risa_a, ri_a)) =
        tin.unpack_renoise_instance_option(tin.renoise_instance_a.clone())?
      {
        if let Some((risa_b, ri_b)) =
          tin.unpack_renoise_instance_option(tin.renoise_instance_b.clone())?
        {
          if let Some(song_a) = static_set.get_song_option(ri_a.loaded_song.clone())? {
            if let Some(song_b) = static_set.get_song_option(ri_b.loaded_song.clone())? {
              // OK IF BOTH A AND B HAVE A SONG

              if let LCXL2InputMessage::Analog(pos, p) = i {
                if pos == LCXL2Position::Knob(8, 2) {
                  let f = (p as f64) / 127.;
                  let bpm = song_a.bpm + (song_b.bpm - song_a.bpm) * f;
                  RenoiseCommunicator::send_message(server, risa_a, MessageToRenoise::SetBPM(bpm))?;
                  RenoiseCommunicator::send_message(server, risa_b, MessageToRenoise::SetBPM(bpm))?;
                  trace!("moved bpm to {}", bpm);
                } else if pos == LCXL2Position::Knob(8, 1) {
                  let f = (p as f64) / 127.;
                  let vol_a = (2. - f * 2.).min(1.);
                  let vol_b = (f * 2.).min(1.);
                  RenoiseCommunicator::send_message(
                    server,
                    risa_a,
                    MessageToRenoise::SetMasterVolume(vol_a),
                  )?;
                  RenoiseCommunicator::send_message(
                    server,
                    risa_b,
                    MessageToRenoise::SetMasterVolume(vol_b),
                  )?;
                  trace!("moved a/b volume to {}", f);
                }
              }
            }
          }
        }
      }
    }

    Ok(())
  }

  pub fn draw(&self, tin: &TinModel, lcxl2: &mut LCXL2Driver) -> Result<()> {
    lcxl2.add(LCXL2Visual::Static(LCXL2Position::Left, 3, 0))?;
    lcxl2.add(LCXL2Visual::Static(LCXL2Position::Right, 3, 0))?;

    if let Some(risa_focus) = tin.renoise_instance_focus {
      lcxl2.add(LCXL2Visual::Static(LCXL2Position::Up, 1, 0))?;
      lcxl2.add(LCXL2Visual::Static(LCXL2Position::Down, 1, 0))?;

      if let Some(risa_a) = tin.renoise_instance_a {
        if risa_a == risa_focus {
          lcxl2.add(LCXL2Visual::Static(LCXL2Position::Up, 3, 0))?;
        }
      }
      if let Some(risa_b) = tin.renoise_instance_b {
        if risa_b == risa_focus {
          lcxl2.add(LCXL2Visual::Static(LCXL2Position::Down, 3, 0))?;
        }
      }
    } else {
      if tin.renoise_instance_a.is_some() {
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Up, 2, 0))?;
      }
      if tin.renoise_instance_b.is_some() {
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Down, 3, 0))?;
      }
    }

    if tin.renoise_instance_a.is_some() {
      if tin.renoise_instance_b.is_some() {
        // let ri_a = tin.renoise_instances.get(risa_a).ok_or(anyhow::Error::msg(
        //   "couldn't find renoise instance in model",
        // ))?;
        // let ri_b = tin.renoise_instances.get(risa_b).ok_or(anyhow::Error::msg(
        //   "couldn't find renoise instance in model",
        // ))?;
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Knob(8, 1), 0, 3))?;
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Knob(8, 2), 3, 3))?;
      }
    }

    Ok(())
  }
}
