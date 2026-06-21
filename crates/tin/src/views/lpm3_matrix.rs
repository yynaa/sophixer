use std::{collections::VecDeque, time::Duration};

use crate::{
  model::{LPM3View, TinModel},
  servers::renoise::RenoiseCommunicator,
};
use anyhow::Result;
use intercom::server::{InterServerCommunicator, udp::UdpServer};
use sophixer_core::{data::buttons::SongButtonAction, messages::renoise::MessageToRenoise};
use tin_drivers_midi::{
  MidiDriver,
  devices::launchpad_mini_mk3::{LPM3Driver, LPM3InputMessage, LPM3Position, LPM3Visual},
};

pub struct ViewLPM3Matrix {
  pub camera: (i64, i64),
}

impl ViewLPM3Matrix {
  pub fn new() -> Self {
    Self { camera: (0, 0) }
  }

  pub fn update(
    &mut self,
    _dt: &Duration,
    tin: &mut TinModel,
    _lpm3: &mut LPM3Driver,
    lpm3_inputs: VecDeque<LPM3InputMessage>,
    server: &UdpServer,
  ) -> Result<()> {
    let static_set = tin.set.clone();
    for i in lpm3_inputs {
      if i == LPM3InputMessage::KeyPressed(LPM3Position::Session) {
        tin.lpm3view = LPM3View::SongList;
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
          // patterns
          for (by, pattern) in &song.patterns {
            let y = *by - self.camera.1;
            if y >= 1 && y <= 7 {
              if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(9, y as u8)) {
                // ri.send_start_next_beat = Some(*by);
                RenoiseCommunicator::send_message(
                  server,
                  rsa,
                  MessageToRenoise::PlaySection(pattern.start),
                )?;
                RenoiseCommunicator::send_message(
                  server,
                  rsa,
                  MessageToRenoise::SetLoop(pattern.loop_start, pattern.loop_end),
                )?;
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
                match &button.action {
                  SongButtonAction::ToggleChannels {
                    channels,
                    default: _,
                    color_off: _,
                    color_on: _,
                  } => {
                    let state = *tin
                      .toggle_button_states
                      .get(&(song_id.clone(), *by, *bx))
                      .ok_or(anyhow::Error::msg(
                        "couldn't find button in toggle state map",
                      ))?;
                    tin
                      .toggle_button_states
                      .insert((song_id.clone(), *by, *bx), !state);
                    for c in channels {
                      RenoiseCommunicator::send_message(
                        server,
                        rsa.clone(),
                        MessageToRenoise::MuteTrack(*c, !state),
                      )?;
                    }
                    trace!("toggled {} channels {:?}", !state, channels);
                  }
                  SongButtonAction::ToggleTrackPatterns {
                    track_patterns,
                    default: _,
                    color_off: _,
                    color_on: _,
                  } => {
                    let state = *tin
                      .toggle_button_states
                      .get(&(song_id.clone(), *by, *bx))
                      .ok_or(anyhow::Error::msg(
                        "couldn't find button in toggle state map",
                      ))?;
                    tin
                      .toggle_button_states
                      .insert((song_id.clone(), *by, *bx), !state);
                    for (t, p) in track_patterns {
                      RenoiseCommunicator::send_message(
                        server,
                        rsa.clone(),
                        MessageToRenoise::MuteTrackSequenceSlot(*t, *p, !state),
                      )?;
                    }
                    trace!("toggled {} track patterns {:?}", !state, track_patterns);
                  }
                  SongButtonAction::ToggleEffectBypass {
                    track,
                    effect,
                    default: _,
                    color_off: _,
                    color_on: _,
                  } => {
                    let state = *tin
                      .toggle_button_states
                      .get(&(song_id.clone(), *by, *bx))
                      .ok_or(anyhow::Error::msg(
                        "couldn't find button in toggle state map",
                      ))?;
                    tin
                      .toggle_button_states
                      .insert((song_id.clone(), *by, *bx), !state);
                    RenoiseCommunicator::send_message(
                      server,
                      rsa.clone(),
                      MessageToRenoise::BypassEffect(*track, *effect, !state),
                    )?;
                    trace!(
                      "toggled {} effect {} bypass on track {}",
                      !state, effect, track
                    );
                  }
                  SongButtonAction::CycleEffectParameterValue {
                    track,
                    effect,
                    default: _,
                    param,
                    cycles,
                  } => {
                    let state = *tin
                      .cycle_button_states
                      .get(&(song_id.clone(), *by, *bx))
                      .ok_or(anyhow::Error::msg(
                        "couldn't find button in toggle state map",
                      ))?;
                    let next_state = (state + 1) % cycles.len();
                    tin
                      .cycle_button_states
                      .insert((song_id.clone(), *by, *bx), next_state);
                    RenoiseCommunicator::send_message(
                      server,
                      rsa.clone(),
                      MessageToRenoise::SetParameterValue(
                        *track,
                        *effect,
                        *param,
                        cycles[next_state].value,
                      ),
                    )?;
                    trace!(
                      "cycled {} on effect {} on track {}",
                      cycles[next_state].value, effect, track
                    );
                  }
                  #[allow(unused)]
                  SongButtonAction::PlaySample {
                    track,
                    pitch,
                    volume,
                    sample,
                    color: _,
                  } => {
                    todo!()
                  }
                }
              }
            }
          }
        }
      }
    }

    Ok(())
  }

  pub fn draw(&self, tin: &TinModel, lpm3: &mut LPM3Driver) -> Result<()> {
    // let directions = [LPM3Position::User, LPM3Position::Keys];
    // for d in directions {
    //   lpm3.add(LPM3Visual::Static(d, 13))?;
    // }
    //
    lpm3.add(LPM3Visual::Static(LPM3Position::Logo, 53))?;
    lpm3.add(LPM3Visual::Static(LPM3Position::Session, 45))?;
    lpm3.add(LPM3Visual::Static(LPM3Position::Keys, 1))?;

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

        if x >= 1 && x < 9 && y >= 1 && y < 9 {
          match &button.action {
            SongButtonAction::ToggleChannels {
              channels: _,
              default: _,
              color_off,
              color_on,
            } => {
              let state = tin
                .toggle_button_states
                .get(&(song_id.clone(), *by, *bx))
                .ok_or(anyhow::Error::msg(
                  "couldn't find button in toggle state map",
                ))?;
              lpm3.add(LPM3Visual::RGB(
                LPM3Position::Grid(x as u8, y as u8),
                if *state { color_on[0] } else { color_off[0] },
                if *state { color_on[1] } else { color_off[1] },
                if *state { color_on[2] } else { color_off[2] },
              ))?;
            }
            SongButtonAction::ToggleTrackPatterns {
              track_patterns: _,
              default: _,
              color_off,
              color_on,
            } => {
              let state = tin
                .toggle_button_states
                .get(&(song_id.clone(), *by, *bx))
                .ok_or(anyhow::Error::msg(
                  "couldn't find button in toggle state map",
                ))?;
              lpm3.add(LPM3Visual::RGB(
                LPM3Position::Grid(x as u8, y as u8),
                if *state { color_on[0] } else { color_off[0] },
                if *state { color_on[1] } else { color_off[1] },
                if *state { color_on[2] } else { color_off[2] },
              ))?;
            }
            SongButtonAction::ToggleEffectBypass {
              track: _,
              effect: _,
              default: _,
              color_off,
              color_on,
            } => {
              let state = tin
                .toggle_button_states
                .get(&(song_id.clone(), *by, *bx))
                .ok_or(anyhow::Error::msg(
                  "couldn't find button in toggle state map",
                ))?;
              lpm3.add(LPM3Visual::RGB(
                LPM3Position::Grid(x as u8, y as u8),
                if *state { color_on[0] } else { color_off[0] },
                if *state { color_on[1] } else { color_off[1] },
                if *state { color_on[2] } else { color_off[2] },
              ))?;
            }
            SongButtonAction::CycleEffectParameterValue {
              track: _,
              effect: _,
              param: _,
              default: _,
              cycles,
            } => {
              let state = *tin
                .cycle_button_states
                .get(&(song_id.clone(), *by, *bx))
                .ok_or(anyhow::Error::msg(
                  "couldn't find button in toggle state map",
                ))?;
              if let Some(cycle) = cycles.get(state) {
                lpm3.add(LPM3Visual::RGB(
                  LPM3Position::Grid(x as u8, y as u8),
                  cycle.color[0],
                  cycle.color[1],
                  cycle.color[2],
                ))?;
              }
            }
            SongButtonAction::PlaySample {
              track: _,
              pitch: _,
              volume: _,
              sample: _,
              color,
            } => {
              lpm3.add(LPM3Visual::RGB(
                LPM3Position::Grid(x as u8, y as u8),
                color[0],
                color[1],
                color[2],
              ))?;
            }
          }
        }
      }
    }

    Ok(())
  }
}
