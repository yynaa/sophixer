use eframe::egui::{ComboBox, DragValue, color_picker::color_edit_button_srgb};
use sophixer_core::data::{
  buttons::{CycleEffectParameterValue, SongButton, SongButtonAction},
  channels::Channel,
};

use crate::{widgets::channel_selector::channel_selector, windows::Window};

pub struct ButtonEditor {
  song_id: String,
  pos: (i64, i64),

  selected_type: ActionType,

  channel_buffer: Channel,
  u64_buffer: u64,
}

impl ButtonEditor {
  pub fn new(song_id: String, pos: (i64, i64), button: &SongButton) -> Self {
    Self {
      song_id,
      pos,

      selected_type: ActionType::from_action(&button.action),

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
          let before = self.selected_type;
          ComboBox::from_label("action type")
            .selected_text(format!("{:?}", self.selected_type))
            .show_ui(ui, |ui| {
              ui.selectable_value(
                &mut self.selected_type,
                ActionType::ToggleChannels,
                "ToggleChannels",
              );
              ui.selectable_value(
                &mut self.selected_type,
                ActionType::ToggleTrackPatterns,
                "ToggleTrackPatterns",
              );
              ui.selectable_value(
                &mut self.selected_type,
                ActionType::ToggleEffectBypass,
                "ToggleEffectBypass",
              );
              ui.selectable_value(
                &mut self.selected_type,
                ActionType::CycleEffectParameterValue,
                "CycleEffectParameterValue",
              );
              ui.selectable_value(
                &mut self.selected_type,
                ActionType::PlaySample,
                "PlaySample",
              );
            });

          if before != self.selected_type {
            button.action = self.selected_type.to_action();
          }

          match &mut button.action {
            SongButtonAction::ToggleTrackPatterns {
              track_patterns,
              default,
              color_off,
              color_on,
            } => {
              // TRACK PATTERNS

              ui.heading("info");

              ui.checkbox(default, "default");

              ui.horizontal(|ui| {
                ui.label("color off");
                color_edit_button_srgb(ui, color_off);
              });

              ui.horizontal(|ui| {
                ui.label("color on");
                color_edit_button_srgb(ui, color_on);
              });

              ui.heading("track patterns");

              let tpclone = track_patterns.clone();
              let mut tps = tpclone.iter().collect::<Vec<&(Channel, u64)>>();
              tps.sort();
              for tp in tps {
                ui.horizontal(|ui| {
                  ui.label(format!("track {} pos {}", tp.0, tp.1));
                  if ui.button("remove").clicked() {
                    track_patterns.remove(tp);
                  }
                });
              }
              ui.horizontal(|ui| {
                channel_selector(&mut self.channel_buffer, ui);
                ui.label("pos");
                ui.add(DragValue::new(&mut self.u64_buffer));
                if ui.button("add").clicked() {
                  track_patterns.insert((self.channel_buffer.clone(), self.u64_buffer));
                }
              });
            }
            SongButtonAction::ToggleChannels {
              channels,
              default,
              color_off,
              color_on,
            } => {
              // CHANNELS

              ui.heading("info");

              ui.checkbox(default, "default");

              ui.horizontal(|ui| {
                ui.label("color off");
                color_edit_button_srgb(ui, color_off);
              });

              ui.horizontal(|ui| {
                ui.label("color on");
                color_edit_button_srgb(ui, color_on);
              });

              ui.heading("track patterns");

              let tpclone = channels.clone();
              let mut tps = tpclone.iter().collect::<Vec<&Channel>>();
              tps.sort();
              for tp in tps {
                ui.horizontal(|ui| {
                  ui.label(format!("track {}", tp));
                  if ui.button("remove").clicked() {
                    channels.remove(tp);
                  }
                });
              }
              ui.horizontal(|ui| {
                channel_selector(&mut self.channel_buffer, ui);
                if ui.button("add").clicked() {
                  channels.insert(self.channel_buffer.clone());
                }
              });
            }
            SongButtonAction::ToggleEffectBypass {
              track,
              effect,
              default,
              color_off,
              color_on,
            } => {
              // EFFECT

              ui.heading("info");

              channel_selector(track, ui);

              ui.horizontal(|ui| {
                ui.label("effect");
                ui.add(DragValue::new(effect));
              });

              ui.checkbox(default, "default");

              ui.horizontal(|ui| {
                ui.label("color off");
                color_edit_button_srgb(ui, color_off);
              });

              ui.horizontal(|ui| {
                ui.label("color on");
                color_edit_button_srgb(ui, color_on);
              });
            }
            SongButtonAction::CycleEffectParameterValue {
              track,
              effect,
              param,
              default,
              cycles,
            } => {
              // CYCLES

              ui.heading("info");

              channel_selector(track, ui);

              ui.horizontal(|ui| {
                ui.label("effect");
                ui.add(DragValue::new(effect));
              });

              ui.horizontal(|ui| {
                ui.label("param");
                ui.add(DragValue::new(param));
              });

              ui.horizontal(|ui| {
                ui.label("default");
                ui.add(DragValue::new(default).range(0..=(cycles.len() - 1)));
              });

              ui.heading("cycles");

              for c in cycles.iter_mut() {
                ui.horizontal(|ui| {
                  ui.label("value");
                  ui.add(DragValue::new(&mut c.value).speed(0.05));
                  ui.label("color");
                  color_edit_button_srgb(ui, &mut c.color);
                });
              }
              ui.horizontal(|ui| {
                if ui.button("-").clicked() && cycles.len() > 0 {
                  cycles.pop();
                }
                if ui.button("+").clicked() {
                  cycles.push(CycleEffectParameterValue::default());
                }
              });
            }
            SongButtonAction::PlaySample {
              track,
              pitch,
              volume,
              sample,
              color,
            } => {
              ui.heading("info");

              channel_selector(track, ui);

              ui.horizontal(|ui| {
                ui.label("pitch");
                ui.add(DragValue::new(pitch));
              });

              ui.horizontal(|ui| {
                ui.label("volume");
                ui.add(DragValue::new(volume));
              });

              ui.horizontal(|ui| {
                ui.label("sample");
                ui.add(DragValue::new(sample));
              });

              ui.horizontal(|ui| {
                ui.label("color");
                color_edit_button_srgb(ui, color);
              });
            }
          }
        }
      }
    }

    Ok(None)
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ActionType {
  ToggleChannels,
  ToggleTrackPatterns,
  ToggleEffectBypass,
  CycleEffectParameterValue,
  PlaySample,
}

impl ActionType {
  fn from_action(action: &SongButtonAction) -> Self {
    match action {
      SongButtonAction::CycleEffectParameterValue {
        track: _,
        effect: _,
        param: _,
        default: _,
        cycles: _,
      } => Self::CycleEffectParameterValue,
      SongButtonAction::ToggleChannels {
        channels: _,
        default: _,
        color_off: _,
        color_on: _,
      } => Self::ToggleChannels,
      SongButtonAction::ToggleEffectBypass {
        track: _,
        effect: _,
        color_off: _,
        color_on: _,
        default: _,
      } => Self::ToggleEffectBypass,
      SongButtonAction::ToggleTrackPatterns {
        track_patterns: _,
        default: _,
        color_off: _,
        color_on: _,
      } => Self::ToggleTrackPatterns,
      SongButtonAction::PlaySample {
        track: _,
        pitch: _,
        volume: _,
        sample: _,
        color: _,
      } => Self::PlaySample,
    }
  }

  fn to_action(self) -> SongButtonAction {
    match self {
      Self::ToggleChannels => SongButtonAction::default_toggle_channels().unwrap(),
      Self::ToggleTrackPatterns => SongButtonAction::default_toggle_track_patterns().unwrap(),
      Self::ToggleEffectBypass => SongButtonAction::default_toggle_effect_bypass().unwrap(),
      Self::CycleEffectParameterValue => {
        SongButtonAction::default_cycle_effect_parameter_value().unwrap()
      }
      Self::PlaySample => SongButtonAction::default_play_sample().unwrap(),
    }
  }
}
