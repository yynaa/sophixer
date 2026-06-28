use std::{collections::VecDeque, time::Duration};

use crate::{model::TinModel, servers::renoise::RenoiseCommunicator};
use anyhow::Result;
use intercom::server::udp::UdpServer;
use sophixer_core::{
  data::channels::Channel,
  messages::renoise::to::{MessageToRenoise, SetBpm, SetParameterValue},
};
use tin_drivers_midi::{
  MidiDriver,
  devices::launch_control_xl_mk2::{LCXL2Driver, LCXL2InputMessage, LCXL2Position, LCXL2Visual},
};

pub struct ViewLCXL2Control {}

impl ViewLCXL2Control {
  pub fn new(_tin: &TinModel) -> Self {
    Self {}
  }

  pub async fn update(
    &mut self,
    _dt: &Duration,
    tin: &mut TinModel,
    _lcxl2: &mut LCXL2Driver,
    lcxl2_inputs: VecDeque<LCXL2InputMessage>,
    server: &UdpServer,
  ) -> Result<()> {
    for i in lcxl2_inputs {
      if let Some(rsa) = tin.renoise_socket {
        for x in 1..=6 {
          if let Some(v) = i.has_analog_moved(LCXL2Position::Knob(x, 3)) {
            RenoiseCommunicator::send_message(
              server,
              rsa,
              MessageToRenoise::build(SetParameterValue {
                track: Channel::Lead(x as u64).to_renoise_number(),
                effect: 2,
                parameter: 1,
                value: (*v as f64) / 128.,
              })?,
            )
            .await?;
          }
          if let Some(v) = i.has_analog_moved(LCXL2Position::Knob(x, 1)) {
            RenoiseCommunicator::send_message(
              server,
              rsa,
              MessageToRenoise::build(SetParameterValue {
                track: Channel::Drum(x as u64).to_renoise_number(),
                effect: 2,
                parameter: 1,
                value: (*v as f64) / 128.,
              })?,
            )
            .await?;
          }

          if let Some(v) = i.has_analog_moved(LCXL2Position::Knob(x, 2)) {
            RenoiseCommunicator::send_message(
              server,
              rsa,
              MessageToRenoise::build(SetParameterValue {
                track: Channel::Drum(x as u64).to_renoise_number(),
                effect: 3,
                parameter: 1,
                value: (*v as f64) / 128.,
              })?,
            )
            .await?;
          }
          if let Some(v) = i.has_analog_moved(LCXL2Position::Slider(x)) {
            RenoiseCommunicator::send_message(
              server,
              rsa,
              MessageToRenoise::build(SetParameterValue {
                track: Channel::Lead(x as u64).to_renoise_number(),
                effect: 3,
                parameter: 1,
                value: (*v as f64) / 128.,
              })?,
            )
            .await?;
          }
        }

        if let Some(v) = i.has_analog_moved(LCXL2Position::Knob(7, 3)) {
          RenoiseCommunicator::send_message(
            server,
            rsa,
            MessageToRenoise::build(SetParameterValue {
              track: Channel::MasterLead.to_renoise_number(),
              effect: 2,
              parameter: 1,
              value: (*v as f64) / 128.,
            })?,
          )
          .await?;
        }
        if let Some(v) = i.has_analog_moved(LCXL2Position::Knob(7, 1)) {
          RenoiseCommunicator::send_message(
            server,
            rsa,
            MessageToRenoise::build(SetParameterValue {
              track: Channel::MasterDrum.to_renoise_number(),
              effect: 2,
              parameter: 1,
              value: (*v as f64) / 128.,
            })?,
          )
          .await?;
        }

        if let Some(v) = i.has_analog_moved(LCXL2Position::Knob(7, 2)) {
          RenoiseCommunicator::send_message(
            server,
            rsa,
            MessageToRenoise::build(SetParameterValue {
              track: Channel::MasterDrum.to_renoise_number(),
              effect: 3,
              parameter: 1,
              value: (*v as f64) / 128.,
            })?,
          )
          .await?;
        }
        if let Some(v) = i.has_analog_moved(LCXL2Position::Slider(7)) {
          RenoiseCommunicator::send_message(
            server,
            rsa,
            MessageToRenoise::build(SetParameterValue {
              track: Channel::MasterLead.to_renoise_number(),
              effect: 3,
              parameter: 1,
              value: (*v as f64) / 128.,
            })?,
          )
          .await?;
        }

        if let Some(v) = i.has_analog_moved(LCXL2Position::Knob(8, 3)) {
          RenoiseCommunicator::send_message(
            server,
            rsa,
            MessageToRenoise::build(SetParameterValue {
              track: Channel::Master.to_renoise_number(),
              effect: 2,
              parameter: 1,
              value: (*v as f64) / 128.,
            })?,
          )
          .await?;
        }

        if let Some(v) = i.has_analog_moved(LCXL2Position::Slider(8)) {
          RenoiseCommunicator::send_message(
            server,
            rsa,
            MessageToRenoise::build(SetParameterValue {
              track: Channel::Master.to_renoise_number(),
              effect: 3,
              parameter: 1,
              value: (*v as f64) / 128.,
            })?,
          )
          .await?;
        }

        if let Some(v) = i.has_analog_moved(LCXL2Position::Knob(8, 2)) {
          let bpm = tin.bpm + (*v as i64 - 64) as f64 * 0.5;
          RenoiseCommunicator::send_message(server, rsa, MessageToRenoise::build(SetBpm { bpm })?)
            .await?;
        }
      }
    }

    Ok(())
  }

  pub fn draw(&self, tin: &TinModel, lcxl2: &mut LCXL2Driver) -> Result<()> {
    if tin.renoise_socket.is_some() {
      for x in 1..=7 {
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Knob(x, 1), 3, 3))?;
      }
      for x in 1..=7 {
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Knob(x, 2), 3, 0))?;
      }
      for x in 1..=8 {
        lcxl2.add(LCXL2Visual::Static(LCXL2Position::Knob(x, 3), 0, 3))?;
      }
      lcxl2.add(LCXL2Visual::Static(LCXL2Position::Knob(8, 2), 1, 3))?;
    }

    Ok(())
  }
}
