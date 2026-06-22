use std::{collections::VecDeque, time::Duration};

use crate::{
  model::{LPM3View, TinModel},
  servers::renoise::RenoiseCommunicator,
};
use anyhow::Result;
use intercom::server::{InterServerCommunicator, udp::UdpServer};
use tin_drivers_midi::{
  MidiDriver,
  devices::launchpad_mini_mk3::{LPM3Driver, LPM3InputMessage, LPM3Position, LPM3Visual},
};

pub struct ViewLPM3SongList {
  cached_song_list: Vec<String>,
  control: bool,
}

impl ViewLPM3SongList {
  pub fn new(tin: &TinModel) -> Self {
    let mut song_list = tin
      .set
      .songs
      .iter()
      .map(|(id, s)| (id.clone(), s.order))
      .collect::<Vec<(String, i64)>>();
    song_list.sort_by(|a, b| a.1.cmp(&b.1));
    let cached_song_list = song_list.iter().map(|(id, _)| id.clone()).collect();

    Self {
      cached_song_list,
      control: false,
    }
  }

  pub fn update(
    &mut self,
    _dt: &Duration,
    tin: &mut TinModel,
    _lpm3: &mut LPM3Driver,
    lpm3_inputs: VecDeque<LPM3InputMessage>,
    server: &UdpServer,
  ) -> Result<()> {
    for i in lpm3_inputs {
      if i == LPM3InputMessage::KeyPressed(LPM3Position::Keys) {
        tin.lpm3view = LPM3View::Matrix;
      }

      if i == LPM3InputMessage::KeyPressed(LPM3Position::SSM) {
        self.control = true;
      }
      if i == LPM3InputMessage::KeyReleased(LPM3Position::SSM) {
        self.control = false;
      }

      if let Some(rsa) = tin.renoise_socket {
        if self.control {
          if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(1, 8)) {
            for (song_id, song) in &tin.set.songs {
              for ((bx, by), button) in &song.buttons {
                let default = button.action.get_default();
                tin
                  .button_states
                  .insert((song_id.clone(), *bx, *by), default);
                let messages = button.action.create_renoise_message(default)?;
                for m in messages {
                  RenoiseCommunicator::send_message(server, rsa, m)?;
                }
              }
            }
          }
        } else {
        }
      }

      for (p, song_id) in self.cached_song_list.iter().enumerate() {
        if tin.set.songs.get(song_id).is_some() {
          let x = (p % 8) + 1;
          let y = p / 8 + 1;
          if y < 8 && i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(x as u8, y as u8)) {
            tin.current_song = Some(song_id.clone());
            info!("loaded song {}", song_id);
          }
        }
      }
    }

    Ok(())
  }

  pub fn draw(&self, tin: &TinModel, lpm3: &mut LPM3Driver) -> Result<()> {
    // navigation
    lpm3.add(LPM3Visual::Static(LPM3Position::Logo, 45))?;
    lpm3.add(LPM3Visual::Static(LPM3Position::Session, 1))?;
    lpm3.add(LPM3Visual::Static(LPM3Position::Keys, 53))?;

    // control
    lpm3.add(LPM3Visual::Static(
      LPM3Position::SSM,
      match self.control {
        false => 13,
        true => 5,
      },
    ))?;

    if let Some(_rsa) = &tin.renoise_socket {
      if self.control {
        // reset all
        lpm3.add(LPM3Visual::Static(LPM3Position::Grid(1, 8), 10))?;
      } else {
      }
    }

    for (i, song_id) in self.cached_song_list.iter().enumerate() {
      if let Some(song) = tin.set.songs.get(song_id) {
        let x = (i % 8) + 1;
        let y = i / 8 + 1;
        if y < 8 {
          lpm3.add(LPM3Visual::RGB(
            LPM3Position::Grid(x as u8, y as u8),
            song.color[0],
            song.color[1],
            song.color[2],
          ))?;
        }
      }
    }

    Ok(())
  }
}
