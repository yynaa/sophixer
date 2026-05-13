use anyhow::Result;
use bimap::BiMap;
use intercom::server::{udp::UdpServer, InterServerCommunicator};
use sophixer_core::{
  messages::renoise::MessageToRenoise,
  song_data::{Set, Song, SongButtonAction},
};
use std::{collections::HashMap, net::SocketAddr};

use crate::servers::renoise::RenoiseCommunicator;

pub struct RenoiseInstance {
  pub loaded_song: Option<String>,
  pub toggle_button_states: HashMap<(i64, i64), bool>,
  pub cycle_button_states: HashMap<(i64, i64), usize>,
}

impl RenoiseInstance {
  pub fn new() -> Self {
    Self {
      loaded_song: None,
      toggle_button_states: HashMap::new(),
      cycle_button_states: HashMap::new(),
    }
  }

  pub fn load_song(
    &mut self,
    addr: &SocketAddr,
    server: &UdpServer,
    song_id: &String,
    song: &Song,
  ) -> Result<()> {
    self.loaded_song = Some(song_id.clone());

    self.toggle_button_states.clear();
    self.cycle_button_states.clear();
    for (y, section) in &song.sections {
      for (x, button) in &section.buttons {
        match &button.action {
          SongButtonAction::ToggleChannels {
            channels,
            default,
            color_off: _,
            color_on: _,
          } => {
            self.toggle_button_states.insert((*y, *x), *default);
            for c in channels {
              RenoiseCommunicator::send_message(
                server,
                addr.clone(),
                MessageToRenoise::MuteTrack(*c, !*default),
              )?;
            }
          }
          SongButtonAction::ToggleTrackPatterns {
            track_patterns,
            default,
            color_off: _,
            color_on: _,
          } => {
            self.toggle_button_states.insert((*y, *x), *default);
            for tp in track_patterns {
              RenoiseCommunicator::send_message(
                server,
                addr.clone(),
                MessageToRenoise::MuteTrackSequenceSlot(tp.0, tp.1, *default),
              )?;
            }
          }
          SongButtonAction::ToggleEffectBypass {
            track,
            effect,
            default,
            color_off: _,
            color_on: _,
          } => {
            self.toggle_button_states.insert((*y, *x), *default);
            RenoiseCommunicator::send_message(
              server,
              addr.clone(),
              MessageToRenoise::BypassEffect(*track, *effect, *default),
            )?;
          }
          SongButtonAction::CycleEffectParameterValue {
            track,
            effect,
            param,
            default,
            cycles,
          } => {
            self.cycle_button_states.insert((*y, *x), *default);
            if let Some(cycle) = cycles.get(*default) {
              RenoiseCommunicator::send_message(
                server,
                addr.clone(),
                MessageToRenoise::SetParameterValue(*track, *effect, *param, cycle.value),
              )?;
            }
          }
        }
      }
    }

    Ok(())
  }
}

pub struct TinModel {
  pub set: Set,
  pub bismuth_instance: Option<SocketAddr>,
  pub renoise_instances: HashMap<SocketAddr, RenoiseInstance>,
  pub renoise_instance_ids: BiMap<u64, SocketAddr>,
  pub renoise_instance_focus: Option<SocketAddr>,
  /// represents the instance A (usually left) used for beatsyncing
  pub renoise_instance_a: Option<SocketAddr>,
  /// represents the instance B (usually right) used for beatsyncing
  pub renoise_instance_b: Option<SocketAddr>,
}

impl TinModel {
  pub fn new(set: Set) -> Self {
    Self {
      set,
      bismuth_instance: None,
      renoise_instances: HashMap::new(),
      renoise_instance_ids: BiMap::new(),
      renoise_instance_focus: None,
      renoise_instance_a: None,
      renoise_instance_b: None,
    }
  }

  pub fn get_mut_renoise_instance(&mut self, addr: SocketAddr) -> Result<&mut RenoiseInstance> {
    let ri = self
      .renoise_instances
      .get_mut(&addr)
      .ok_or(anyhow::Error::msg(
        "couldn't find renoise instance in model",
      ))?;
    Ok(ri)
  }

  pub fn get_renoise_instance_option(
    &self,
    addr_opt: Option<SocketAddr>,
  ) -> Result<Option<&RenoiseInstance>> {
    if let Some(addr) = addr_opt {
      let ri = self.renoise_instances.get(&addr).ok_or(anyhow::Error::msg(
        "couldn't find renoise instance in model",
      ))?;
      Ok(Some(ri))
    } else {
      Ok(None)
    }
  }

  pub fn unpack_renoise_instance_option(
    &self,
    addr_opt: Option<SocketAddr>,
  ) -> Result<Option<(SocketAddr, &RenoiseInstance)>> {
    if let Some(addr) = addr_opt {
      let ri = self.renoise_instances.get(&addr).ok_or(anyhow::Error::msg(
        "couldn't find renoise instance in model",
      ))?;
      Ok(Some((addr.clone(), ri)))
    } else {
      Ok(None)
    }
  }

  pub fn unpack_mut_renoise_instance_option(
    &mut self,
    addr_opt: Option<SocketAddr>,
  ) -> Result<Option<(SocketAddr, &mut RenoiseInstance)>> {
    if let Some(addr) = addr_opt {
      let ri = self
        .renoise_instances
        .get_mut(&addr)
        .ok_or(anyhow::Error::msg(
          "couldn't find renoise instance in model",
        ))?;
      Ok(Some((addr.clone(), ri)))
    } else {
      Ok(None)
    }
  }
}
