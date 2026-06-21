use eframe::egui::{DragValue, color_picker::color_edit_button_srgb};
use sophixer_core::data::SongPattern;

use crate::windows::Window;

pub struct PatternEditor {
  song_id: String,
  pattern_id: i64,
}

impl PatternEditor {
  pub fn new(song_id: String, pattern_id: i64, _section: &SongPattern) -> Self {
    Self {
      song_id,
      pattern_id,
    }
  }
}
impl Window for PatternEditor {
  fn title(&mut self) -> String {
    format!("section editor: {}@{}", self.song_id, self.pattern_id)
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
          song.patterns.remove(&self.pattern_id);
        }

        if let Some(pattern) = song.patterns.get_mut(&self.pattern_id) {
          ui.heading("info");

          ui.horizontal(|ui| {
            ui.label("start");
            ui.add(DragValue::new(&mut pattern.start));
          });
          ui.horizontal(|ui| {
            ui.label("loop start");
            ui.add(DragValue::new(&mut pattern.loop_start));
          });
          ui.horizontal(|ui| {
            ui.label("loop end");
            ui.add(DragValue::new(&mut pattern.loop_end));
          });
          ui.horizontal(|ui| {
            ui.label("color");
            color_edit_button_srgb(ui, &mut pattern.color);
          });
        }
      }
    }

    Ok(None)
  }
}
