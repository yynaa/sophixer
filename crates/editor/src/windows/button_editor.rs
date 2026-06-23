use eframe::egui::{ComboBox, DragValue, color_picker::color_edit_button_srgb};
use sophixer_core::data::{
  buttons::{
    SongButton, SongButtonAction,
    cycle_effect_parameter_value::{CycleEffectParameterValue, ParameterValue},
    play_sample::PlaySample,
    toggle_channels::ToggleChannels,
    toggle_effect_bypass::ToggleEffectBypass,
    toggle_track_patterns::ToggleTrackPatterns,
  },
  channels::Channel,
};

use crate::{widgets::channel_selector::channel_selector, windows::Window};

pub struct ButtonEditor {
  song_id: String,
  pos: (i64, i64),

  channel_buffer: Channel,
  u64_buffer: u64,
}

impl ButtonEditor {
  pub fn new(song_id: String, pos: (i64, i64), _button: &SongButton) -> Self {
    Self {
      song_id,
      pos,

      channel_buffer: Channel::Master,
      u64_buffer: 0,
    }
  }
}

impl Window for ButtonEditor {
  fn title(&mut self) -> String {
    format!(
      "button editor: {}@{},{}",
      self.song_id, self.pos.0, self.pos.1,
    )
  }

  fn ui(
    &mut self,
    model: &mut crate::Model,
    ui: &mut eframe::egui::Ui,
  ) -> anyhow::Result<Option<Box<dyn Window>>> {
    if let Some(set) = &mut model.set {
      if let Some(song) = set.songs.get_mut(&self.song_id) {
        ui.heading("manage");

        if ui.button("delete").clicked() {
          song.buttons.remove(&self.pos);
        }

        if let Some(button) = song.buttons.get_mut(&self.pos) {
          ComboBox::from_label("action type")
            .selected_text(format!("{}", button.action))
            .show_ui(ui, |ui| {
              ui.selectable_value(
                &mut button.action,
                SongButtonAction::ToggleChannels(ToggleChannels::default()),
                "ToggleChannels",
              );
              ui.selectable_value(
                &mut button.action,
                SongButtonAction::ToggleTrackPatterns(ToggleTrackPatterns::default()),
                "ToggleTrackPatterns",
              );
              ui.selectable_value(
                &mut button.action,
                SongButtonAction::ToggleEffectBypass(ToggleEffectBypass::default()),
                "ToggleEffectBypass",
              );
              ui.selectable_value(
                &mut button.action,
                SongButtonAction::CycleEffectParameterValue(CycleEffectParameterValue::default()),
                "CycleEffectParameterValue",
              );
              ui.selectable_value(
                &mut button.action,
                SongButtonAction::PlaySample(PlaySample::default()),
                "PlaySample",
              );
            });

          match &mut button.action {
            SongButtonAction::ToggleTrackPatterns(inner) => {
              // TRACK PATTERNS

              ui.heading("info");

              ui.checkbox(&mut inner.default, "default");

              ui.horizontal(|ui| {
                ui.label("color off");
                color_edit_button_srgb(ui, &mut inner.color_off);
              });

              ui.horizontal(|ui| {
                ui.label("color on");
                color_edit_button_srgb(ui, &mut inner.color_on);
              });

              ui.heading("track patterns");

              let tpclone = inner.track_patterns.clone();
              let mut tps = tpclone.iter().collect::<Vec<&(Channel, u64)>>();
              tps.sort();
              for tp in tps {
                ui.horizontal(|ui| {
                  ui.label(format!("track {} pos {}", tp.0, tp.1));
                  if ui.button("remove").clicked() {
                    inner.track_patterns.remove(tp);
                  }
                });
              }
              ui.horizontal(|ui| {
                channel_selector(&mut self.channel_buffer, ui);
                ui.label("pos");
                ui.add(DragValue::new(&mut self.u64_buffer));
                if ui.button("add").clicked() {
                  inner
                    .track_patterns
                    .insert((self.channel_buffer.clone(), self.u64_buffer));
                }
              });
            }
            SongButtonAction::ToggleChannels(inner) => {
              // CHANNELS

              ui.heading("info");

              ui.checkbox(&mut inner.default, "default");

              ui.horizontal(|ui| {
                ui.label("color off");
                color_edit_button_srgb(ui, &mut inner.color_off);
              });

              ui.horizontal(|ui| {
                ui.label("color on");
                color_edit_button_srgb(ui, &mut inner.color_on);
              });

              ui.heading("track patterns");

              let tpclone = inner.channels.clone();
              let mut tps = tpclone.iter().collect::<Vec<&Channel>>();
              tps.sort();
              for tp in tps {
                ui.horizontal(|ui| {
                  ui.label(format!("track {}", tp));
                  if ui.button("remove").clicked() {
                    inner.channels.remove(tp);
                  }
                });
              }
              ui.horizontal(|ui| {
                channel_selector(&mut self.channel_buffer, ui);
                if ui.button("add").clicked() {
                  inner.channels.insert(self.channel_buffer.clone());
                }
              });
            }
            SongButtonAction::ToggleEffectBypass(inner) => {
              // EFFECT

              ui.heading("info");

              channel_selector(&mut inner.track, ui);

              ui.horizontal(|ui| {
                ui.label("effect");
                ui.add(DragValue::new(&mut inner.effect));
              });

              ui.checkbox(&mut inner.default, "default");

              ui.horizontal(|ui| {
                ui.label("color off");
                color_edit_button_srgb(ui, &mut inner.color_off);
              });

              ui.horizontal(|ui| {
                ui.label("color on");
                color_edit_button_srgb(ui, &mut inner.color_on);
              });
            }
            SongButtonAction::CycleEffectParameterValue(inner) => {
              // CYCLES

              ui.heading("info");

              channel_selector(&mut inner.track, ui);

              ui.horizontal(|ui| {
                ui.label("effect");
                ui.add(DragValue::new(&mut inner.effect));
              });

              ui.horizontal(|ui| {
                ui.label("param");
                ui.add(DragValue::new(&mut inner.param));
              });

              ui.horizontal(|ui| {
                ui.label("default");

                let range_max = match inner.cycles.len() {
                  0 => 0,
                  a => a - 1,
                };
                ui.add(DragValue::new(&mut inner.default).range(0..=range_max));
              });

              ui.heading("cycles");

              for c in inner.cycles.iter_mut() {
                ui.horizontal(|ui| {
                  ui.label("value");
                  ui.add(DragValue::new(&mut c.value).speed(0.05));
                  ui.label("color");
                  color_edit_button_srgb(ui, &mut c.color);
                });
              }
              ui.horizontal(|ui| {
                if ui.button("-").clicked() && inner.cycles.len() > 0 {
                  inner.cycles.pop();
                }
                if ui.button("+").clicked() {
                  inner.cycles.push(ParameterValue::default());
                }
              });
            }
            SongButtonAction::PlaySample(inner) => {
              ui.heading("info");

              channel_selector(&mut inner.track, ui);

              ui.horizontal(|ui| {
                ui.label("pitch");
                ui.add(DragValue::new(&mut inner.pitch));
              });

              ui.horizontal(|ui| {
                ui.label("volume");
                ui.add(DragValue::new(&mut inner.volume));
              });

              ui.horizontal(|ui| {
                ui.label("sample");
                ui.add(DragValue::new(&mut inner.sample));
              });

              ui.horizontal(|ui| {
                ui.label("color");
                color_edit_button_srgb(ui, &mut inner.color);
              });
            }
          }
        }
      }
    }

    Ok(None)
  }
}

// #[derive(Debug, PartialEq, Clone, Copy)]
// enum ActionType {
//   ToggleChannels,
//   ToggleTrackPatterns,
//   ToggleEffectBypass,
//   CycleEffectParameterValue,
//   PlaySample,
// }

// impl ActionType {
//   fn from_action(action: &SongButtonAction) -> Self {
//     match action {
//       SongButtonAction::CycleEffectParameterValue(_) => Self::CycleEffectParameterValue,
//       SongButtonAction::ToggleChannels(_) => Self::ToggleChannels,
//       SongButtonAction::ToggleEffectBypass(_) => Self::ToggleEffectBypass,
//       SongButtonAction::ToggleTrackPatterns(_) => Self::ToggleTrackPatterns,
//       SongButtonAction::PlaySample(_) => Self::PlaySample,
//     }
//   }

//   fn to_action(self) -> SongButtonAction {
//     match self {
//       Self::ToggleChannels => SongButtonAction::ToggleChannels(ToggleChannels::default()),
//       Self::ToggleTrackPatterns => {
//         SongButtonAction::ToggleTrackPatterns(ToggleTrackPatterns::default())
//       }
//       Self::ToggleEffectBypass => SongButtonAction::ToggleEffectBypass(ToggleEffectBypass),
//       Self::CycleEffectParameterValue => {
//         SongButtonAction::default_cycle_effect_parameter_value().unwrap()
//       }
//       Self::PlaySample => SongButtonAction::default_play_sample().unwrap(),
//     }
//   }
// }
