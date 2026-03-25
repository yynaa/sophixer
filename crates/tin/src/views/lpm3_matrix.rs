use std::{collections::VecDeque, time::Duration};

use crate::model::TinModel;
use anyhow::Result;
use sophixer_core::song_data::SongButtonAction;
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
    dt: &Duration,
    tin: &mut TinModel,
    lpm3: &mut LPM3Driver,
    lpm3_inputs: VecDeque<LPM3InputMessage>,
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

      if let Some(risa) = &tin.renoise_instance_focus {
        let ri = tin
          .renoise_instances
          .get_mut(risa)
          .ok_or(anyhow::Error::msg(
            "couldn't find renoise instance in model",
          ))?;
        if let Some(song_id) = &ri.loaded_song {
          let song = tin
            .set
            .songs
            .get(song_id)
            .ok_or(anyhow::Error::msg("couldn't find song in model"))?;

          for (by, section) in &song.sections {
            for (bx, button) in &section.buttons {
              let x = *bx - self.camera.0;
              let y = *by - self.camera.1;

              if !(x < 1 || x > 8 || y < 1 || y > 8) {
                if i == LPM3InputMessage::KeyPressed(LPM3Position::Grid(x as u8, y as u8)) {
                  // matrix button pressed
                  match button.action {
                    SongButtonAction::ToggleChannels {
                      channels: _,
                      instant: _,
                      color_off: _,
                      color_on: _,
                    } => {
                      let state =
                        ri.toggle_button_states
                          .get(&(*by, *bx))
                          .ok_or(anyhow::Error::msg(
                            "couldn't find button in toggle state map",
                          ))?;
                      ri.toggle_button_states.insert((*by, *bx), !state);
                    }
                    SongButtonAction::ToggleTrackPatterns {
                      track_patterns: _,
                      instant: _,
                      color_off: _,
                      color_on: _,
                    } => {
                      let state =
                        ri.toggle_button_states
                          .get(&(*by, *bx))
                          .ok_or(anyhow::Error::msg(
                            "couldn't find button in toggle state map",
                          ))?;
                      ri.toggle_button_states.insert((*by, *bx), !state);
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
    if let Some(risa) = &tin.renoise_instance_focus {
      let ri = tin.renoise_instances.get(risa).ok_or(anyhow::Error::msg(
        "couldn't find renoise instance in model",
      ))?;
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
                match button.action {
                  SongButtonAction::ToggleChannels {
                    channels: _,
                    instant: _,
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
                    instant: _,
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
