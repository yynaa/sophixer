use std::{collections::VecDeque, net::SocketAddr, time::Duration};

use crate::{
  model::{RenoiseInstance, TinModel},
  servers::renoise::RenoiseCommunicator,
};
use anyhow::Result;
use intercom::server::{udp::UdpServer, InterServerCommunicator};
use sophixer_core::{messages::renoise::MessageToRenoise, song_data::SongButtonAction};
use tin_drivers_midi::{
  devices::launchpad_mini_mk3::{LPM3Driver, LPM3InputMessage, LPM3Position, LPM3Visual},
  MidiDriver,
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
    for i in lpm3_inputs {
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

      if i == LPM3InputMessage::KeyPressed(LPM3Position::Keys) {
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
      if i == LPM3InputMessage::KeyPressed(LPM3Position::User) {
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

      if let Some(risa) = &tin.renoise_instance_focus {
        let ri = tin
          .renoise_instances
          .get_mut(risa)
          .ok_or(anyhow::Error::msg(
            "couldn't find renoise instance in model",
          ))?;
        if i == LPM3InputMessage::KeyPressed(LPM3Position::Session) {
          RenoiseCommunicator::send_message(server, risa.clone(), MessageToRenoise::StopTransport)?;
        }
        if let Some(song_id) = &ri.loaded_song {
          let song = tin
            .set
            .songs
            .get(song_id)
            .ok_or(anyhow::Error::msg("couldn't find song in model"))?;

          for (by, section) in &song.sections {
            let y = *by - self.camera.1;
            if y >= 1 && y < 9 {
              if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(9, y as u8)) {
                RenoiseCommunicator::send_message(
                  server,
                  risa.clone(),
                  MessageToRenoise::PlaySection(section.start, section.length),
                )?;
              }

              for (bx, button) in &section.buttons {
                let x = *bx - self.camera.0;
                if x >= 1 && x < 9 {
                  if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(x as u8, y as u8)) {
                    // matrix button pressed
                    match &button.action {
                      SongButtonAction::ToggleChannels {
                        channels,
                        default: _,
                        color_off: _,
                        color_on: _,
                      } => {
                        let state =
                          *ri
                            .toggle_button_states
                            .get(&(*by, *bx))
                            .ok_or(anyhow::Error::msg(
                              "couldn't find button in toggle state map",
                            ))?;
                        ri.toggle_button_states.insert((*by, *bx), !state);
                        for c in channels {
                          RenoiseCommunicator::send_message(
                            server,
                            risa.clone(),
                            MessageToRenoise::MuteTrack(*c, state),
                          )?;
                        }
                      }
                      SongButtonAction::ToggleTrackPatterns {
                        track_patterns,
                        default: _,
                        color_off: _,
                        color_on: _,
                      } => {
                        let state =
                          *ri
                            .toggle_button_states
                            .get(&(*by, *bx))
                            .ok_or(anyhow::Error::msg(
                              "couldn't find button in toggle state map",
                            ))?;
                        ri.toggle_button_states.insert((*by, *bx), !state);
                        for (t, p) in track_patterns {
                          RenoiseCommunicator::send_message(
                            server,
                            risa.clone(),
                            MessageToRenoise::MuteTrackSequenceSlot(*t, *p, !state),
                          )?;
                        }
                      }
                      SongButtonAction::ToggleEffectBypass {
                        track,
                        effect,
                        default: _,
                        color_off: _,
                        color_on: _,
                      } => {
                        let state =
                          *ri
                            .toggle_button_states
                            .get(&(*by, *bx))
                            .ok_or(anyhow::Error::msg(
                              "couldn't find button in toggle state map",
                            ))?;
                        ri.toggle_button_states.insert((*by, *bx), !state);
                        RenoiseCommunicator::send_message(
                          server,
                          risa.clone(),
                          MessageToRenoise::BypassEffect(*track, *effect, !state),
                        )?;
                      }
                      SongButtonAction::CycleEffectParameterValue {
                        track,
                        effect,
                        default: _,
                        param,
                        cycles,
                      } => {
                        let state =
                          *ri
                            .cycle_button_states
                            .get(&(*by, *bx))
                            .ok_or(anyhow::Error::msg(
                              "couldn't find button in toggle state map",
                            ))?;
                        let next_state = (state + 1) % cycles.len();
                        ri.cycle_button_states.insert((*by, *bx), next_state);
                        RenoiseCommunicator::send_message(
                          server,
                          risa.clone(),
                          MessageToRenoise::SetParameterValue(
                            *track,
                            *effect,
                            *param,
                            cycles[next_state].value,
                          ),
                        )?;
                      }
                    }
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
    let directions = [LPM3Position::User, LPM3Position::Keys];
    for d in directions {
      lpm3.add(LPM3Visual::Static(d, 13))?;
    }

    if let Some(risa) = &tin.renoise_instance_focus {
      let ri = tin.renoise_instances.get(risa).ok_or(anyhow::Error::msg(
        "couldn't find renoise instance in model",
      ))?;
      lpm3.add(LPM3Visual::Static(LPM3Position::Session, 5))?;
      if let Some(song_id) = &ri.loaded_song {
        // STATUS
        lpm3.add(LPM3Visual::Static(LPM3Position::Logo, 37))?;

        let song = tin
          .set
          .songs
          .get(song_id)
          .ok_or(anyhow::Error::msg("couldn't find song in model"))?;

        // CONTROL PANEL
        let directions = [
          LPM3Position::Up,
          LPM3Position::Down,
          LPM3Position::Left,
          LPM3Position::Right,
        ];
        for d in directions {
          lpm3.add(LPM3Visual::RGB(d, song.color.0, song.color.1, song.color.2))?;
        }

        // sections
        for (by, section) in &song.sections {
          let y = *by - self.camera.1;
          if y >= 1 && y < 9 {
            // section player
            lpm3.add(LPM3Visual::RGB(
              LPM3Position::Grid(9, y as u8),
              section.color.0,
              section.color.1,
              section.color.2,
            ))?;

            // buttons
            for (bx, button) in &section.buttons {
              let x = *bx - self.camera.0;

              if x >= 1 && x < 9 {
                match &button.action {
                  SongButtonAction::ToggleChannels {
                    channels: _,
                    default: _,
                    color_off,
                    color_on,
                  } => {
                    let state =
                      ri.toggle_button_states
                        .get(&(*by, *bx))
                        .ok_or(anyhow::Error::msg(
                          "couldn't find button in toggle state map",
                        ))?;
                    lpm3.add(LPM3Visual::RGB(
                      LPM3Position::Grid(x as u8, y as u8),
                      if *state { color_on.0 } else { color_off.0 },
                      if *state { color_on.1 } else { color_off.1 },
                      if *state { color_on.2 } else { color_off.2 },
                    ))?;
                  }
                  SongButtonAction::ToggleTrackPatterns {
                    track_patterns: _,
                    default: _,
                    color_off,
                    color_on,
                  } => {
                    let state =
                      ri.toggle_button_states
                        .get(&(*by, *bx))
                        .ok_or(anyhow::Error::msg(
                          "couldn't find button in toggle state map",
                        ))?;
                    lpm3.add(LPM3Visual::RGB(
                      LPM3Position::Grid(x as u8, y as u8),
                      if *state { color_on.0 } else { color_off.0 },
                      if *state { color_on.1 } else { color_off.1 },
                      if *state { color_on.2 } else { color_off.2 },
                    ))?;
                  }
                  SongButtonAction::ToggleEffectBypass {
                    track: _,
                    effect: _,
                    default: _,
                    color_off,
                    color_on,
                  } => {
                    let state =
                      ri.toggle_button_states
                        .get(&(*by, *bx))
                        .ok_or(anyhow::Error::msg(
                          "couldn't find button in toggle state map",
                        ))?;
                    lpm3.add(LPM3Visual::RGB(
                      LPM3Position::Grid(x as u8, y as u8),
                      if *state { color_on.0 } else { color_off.0 },
                      if *state { color_on.1 } else { color_off.1 },
                      if *state { color_on.2 } else { color_off.2 },
                    ))?;
                  }
                  SongButtonAction::CycleEffectParameterValue {
                    track: _,
                    effect: _,
                    param: _,
                    default: _,
                    cycles,
                  } => {
                    let state =
                      *ri
                        .cycle_button_states
                        .get(&(*by, *bx))
                        .ok_or(anyhow::Error::msg(
                          "couldn't find button in toggle state map",
                        ))?;
                    if let Some(cycle) = cycles.get(state) {
                      lpm3.add(LPM3Visual::RGB(
                        LPM3Position::Grid(x as u8, y as u8),
                        cycle.color.0,
                        cycle.color.1,
                        cycle.color.2,
                      ))?;
                    }
                  }
                }
              }
            }
          }
        }
      } else {
        lpm3.add(LPM3Visual::Flashing(LPM3Position::Logo, 37, 13))?;
      }
    } else {
      lpm3.add(LPM3Visual::Flashing(LPM3Position::Logo, 37, 5))?;
    }

    Ok(())
  }
}
