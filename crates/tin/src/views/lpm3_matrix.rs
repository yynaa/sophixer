use std::{collections::VecDeque, time::Duration};

use crate::{
  model::{LPM3View, TinModel},
  servers::renoise::RenoiseCommunicator,
};
use anyhow::Result;
use intercom::server::udp::UdpServer;
use sophixer_core::{
  data::buttons::ActionDescriptor,
  messages::renoise::to::{MessageToRenoise, PlaySection, SetBpm, SetLoop, StopTransport},
};
use tin_drivers_midi::{
  MidiDriver, MidiPhysicalState,
  devices::launchpad_mini_mk3::{LPM3Driver, LPM3InputMessage, LPM3Position, LPM3Visual},
};

pub struct ViewLPM3Matrix {
  pub camera: (i64, i64),

  control: bool,

  insta_play: bool,
}

impl ViewLPM3Matrix {
  pub fn new() -> Self {
    Self {
      camera: (0, 0),
      control: false,
      insta_play: false,
    }
  }

  pub async fn update(
    &mut self,
    _dt: &Duration,
    tin: &mut TinModel,
    lpm3: &mut LPM3Driver,
    lpm3_inputs: VecDeque<LPM3InputMessage>,
    server: &UdpServer,
  ) -> Result<()> {
    let static_set = tin.set.clone();
    for i in lpm3_inputs {
      if i == LPM3InputMessage::KeyPressed(LPM3Position::Session) {
        tin.lpm3view = LPM3View::SongList;
      }

      if let MidiPhysicalState::Binary(b) = lpm3.get_position_state(LPM3Position::SSM)? {
        self.control = b;
      }
      if let MidiPhysicalState::Binary(b) = lpm3.get_position_state(LPM3Position::Grid(1, 8))? {
        self.insta_play = b && !self.control;
      }

      if i == LPM3InputMessage::KeyPressed(LPM3Position::Left) {
        self.camera.0 -= 1;
      }
      if i == LPM3InputMessage::KeyPressed(LPM3Position::Right) {
        self.camera.0 += 1;
      }
      if i == LPM3InputMessage::KeyPressed(LPM3Position::Up) {
        self.camera.1 -= 1;
      }
      if i == LPM3InputMessage::KeyPressed(LPM3Position::Down) {
        self.camera.1 += 1;
      }

      if let Some(rsa) = tin.renoise_socket {
        if let Some(song_id) = tin.current_song.clone()
          && let Some(song) = static_set.get_song_option(tin.current_song.clone())?
        {
          // control
          if self.control {
            if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(1, 8)) {
              for ((bx, by), button) in &song.buttons {
                let default = button.action.get_default();
                tin
                  .button_states
                  .insert((song_id.clone(), *bx, *by), default);
                let messages = button.action.create_renoise_message(default)?;
                for m in messages {
                  RenoiseCommunicator::send_message(server, rsa, m).await?;
                }
              }
            }
            if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(2, 8)) {
              RenoiseCommunicator::send_message(
                server,
                rsa,
                MessageToRenoise::build(StopTransport {})?,
              )
              .await?;
            }
          } else {
            if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(2, 8)) {
              RenoiseCommunicator::send_message(
                server,
                rsa,
                MessageToRenoise::build(PlaySection {
                  section: tin.set.stop_seq_pos,
                  force_play: false,
                })?,
              )
              .await?;
              RenoiseCommunicator::send_message(
                server,
                rsa,
                MessageToRenoise::build(SetLoop {
                  start: tin.set.stop_seq_pos,
                  end: tin.set.stop_seq_pos,
                })?,
              )
              .await?;
            }
            if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(3, 8)) {
              tin.bpm = song.bpm;
              RenoiseCommunicator::send_message(
                server,
                rsa,
                MessageToRenoise::build(SetBpm { bpm: tin.bpm })?,
              )
              .await?;
            }
          }

          // patterns
          for (by, pattern) in &song.patterns {
            let y = *by - self.camera.1;
            if y >= 1 && y <= 7 {
              if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(9, y as u8)) {
                RenoiseCommunicator::send_message(
                  server,
                  rsa,
                  MessageToRenoise::build(PlaySection {
                    section: pattern.start,
                    force_play: self.insta_play,
                  })?,
                )
                .await?;
                RenoiseCommunicator::send_message(
                  server,
                  rsa,
                  MessageToRenoise::build(SetLoop {
                    start: pattern.loop_start,
                    end: pattern.loop_end,
                  })?,
                )
                .await?;
                trace!(
                  "playing pattern start {} loop_start {} loop_end {}",
                  pattern.start, pattern.loop_start, pattern.loop_end
                );
              }
            }
          }

          // buttons
          for ((bx, by), button) in &song.buttons {
            let x = *bx - self.camera.0;
            let y = *by - self.camera.1;
            if x >= 1 && x < 9 && y >= 1 && y < 9 {
              if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(x as u8, y as u8)) {
                // matrix button pressed
                let key = (song_id.clone(), *bx, *by);
                let current_state = tin
                  .button_states
                  .get(&key)
                  .ok_or(anyhow::Error::msg("couldn't find state in model"))?;
                let next = button.action.next(current_state.clone())?;
                let messages = button.action.create_renoise_message(next)?;
                for m in messages {
                  RenoiseCommunicator::send_message(server, rsa, m).await?;
                }
                tin.button_states.insert(key, next);
              }
            }
          }
        }
      }
    }

    Ok(())
  }

  pub fn draw(&self, tin: &TinModel, lpm3: &mut LPM3Driver) -> Result<()> {
    // navigation
    lpm3.add(LPM3Visual::Static(LPM3Position::Logo, 53))?;
    lpm3.add(LPM3Visual::Static(LPM3Position::Session, 45))?;
    lpm3.add(LPM3Visual::Static(LPM3Position::Keys, 1))?;

    // control
    lpm3.add(LPM3Visual::Static(
      LPM3Position::SSM,
      match self.control {
        false => 13,
        true => 5,
      },
    ))?;

    if let Some(song_id) = tin.current_song.clone()
      && let Some(song) = tin.set.songs.get(&song_id)
    {
      // CONTROL PANEL
      let directions = [
        LPM3Position::Up,
        LPM3Position::Down,
        LPM3Position::Left,
        LPM3Position::Right,
      ];
      for d in directions {
        lpm3.add(LPM3Visual::RGB(
          d,
          song.color[0],
          song.color[1],
          song.color[2],
        ))?;
      }

      // control
      if tin.renoise_socket.is_some() {
        if self.control {
          // reset
          lpm3.add(LPM3Visual::Static(LPM3Position::Grid(1, 8), 9))?;
          // stop transport
          lpm3.add(LPM3Visual::Static(LPM3Position::Grid(2, 8), 5))?;
        } else {
          // instaplay
          lpm3.add(LPM3Visual::Static(LPM3Position::Grid(1, 8), 69))?;
          // go to break
          lpm3.add(LPM3Visual::Static(LPM3Position::Grid(2, 8), 1))?;
          // sync bpm
          lpm3.add(LPM3Visual::Flashing(LPM3Position::Grid(3, 8), 5, 21))?;
        }
      }

      // sections
      for (by, pattern) in &song.patterns {
        let y = *by - self.camera.1;
        if y >= 1 && y <= 7 {
          // section player
          lpm3.add(LPM3Visual::RGB(
            LPM3Position::Grid(9, y as u8),
            pattern.color[0],
            pattern.color[1],
            pattern.color[2],
          ))?;
        }
      }

      // buttons
      for ((bx, by), button) in &song.buttons {
        let x = *bx - self.camera.0;
        let y = *by - self.camera.1;

        if x >= 1 && x < 9 && y >= 1 && y < 8 {
          let current_state = tin
            .button_states
            .get(&(song_id.clone(), *bx, *by))
            .ok_or(anyhow::Error::msg("couldn't find state in model"))?;
          let color = button.action.get_color(current_state.clone())?;
          lpm3.add(LPM3Visual::RGB(
            LPM3Position::Grid(x as u8, y as u8),
            color[0],
            color[1],
            color[2],
          ))?;
        }
      }
    }

    Ok(())
  }
}
