use eframe::egui::{DragValue, color_picker::color_edit_button_srgb};
use sophixer_core::data::{
  Song, SongPattern,
  buttons::{SongButton, SongButtonAction},
};

use crate::windows::{Window, button_editor::ButtonEditor, pattern_editor::PatternEditor};

pub struct SongEditor {
  song_id: String,
  new_pattern: i64,
  new_button_x: i64,
  new_button_y: i64,
}

impl SongEditor {
  pub fn new(song_id: String, _song: &Song) -> Self {
    Self {
      song_id,
      new_pattern: 1,
      new_button_x: 1,
      new_button_y: 1,
    }
  }
}

impl<'a> Window for SongEditor {
  fn title(&mut self) -> String {
    format!("song editor: {}", self.song_id)
  }

  fn ui(
    &mut self,
    model: &mut crate::Model,
    ui: &mut eframe::egui::Ui,
  ) -> anyhow::Result<Option<Box<dyn Window>>> {
    let mut n: Option<Box<dyn Window>> = None;

    if let Some(set) = &mut model.set {
      ui.heading("manage");

      if ui.button("delete").clicked() {
        set.songs.remove(&self.song_id);
      }

      if let Some(song) = set.songs.get_mut(&self.song_id) {
        ui.heading("info");

        ui.horizontal(|ui| {
          ui.label("name");
          ui.text_edit_singleline(&mut song.name);
        });

        ui.horizontal(|ui| {
          ui.label("authors");
          ui.text_edit_singleline(&mut song.authors);
        });

        ui.horizontal(|ui| {
          ui.label("order");
          ui.add(DragValue::new(&mut song.order));
        });

        ui.horizontal(|ui| {
          ui.label("color");
          color_edit_button_srgb(ui, &mut song.color);
        });

        ui.heading("patterns");

        let mut sorted_patterns = song.patterns.keys().map(|f| *f).collect::<Vec<i64>>();
        sorted_patterns.sort();

        for secid in sorted_patterns {
          if ui.button(format!("section @ line {}", secid)).clicked() {
            if let Some(pattern) = song.patterns.get(&secid) {
              n = Some(Box::new(PatternEditor::new(
                self.song_id.clone(),
                secid,
                pattern,
              )));
            }
          }
        }

        ui.horizontal(|ui| {
          ui.add(DragValue::new(&mut self.new_pattern));
          if ui.button("..new").clicked() {
            song
              .patterns
              .insert(self.new_pattern, SongPattern::default());
          }
        });

        ui.heading("buttons");

        let mut sorted_buttons = song.buttons.keys().map(|f| *f).collect::<Vec<(i64, i64)>>();
        sorted_buttons.sort();

        for (button_x, button_y) in sorted_buttons {
          if ui
            .button(format!("button @ pos {},{}", button_x, button_y))
            .clicked()
          {
            if let Some(button) = song.buttons.get(&(button_x, button_y)) {
              n = Some(Box::new(ButtonEditor::new(
                self.song_id.clone(),
                (button_x, button_y),
                &button,
              )));
            }
          }
        }

        ui.horizontal(|ui| {
          ui.add(DragValue::new(&mut self.new_button_x));
          ui.add(DragValue::new(&mut self.new_button_y));
          if ui.button("..new").clicked() {
            song.buttons.insert(
              (self.new_button_x, self.new_button_y),
              SongButton::new(SongButtonAction::default_play_sample().unwrap()).unwrap(),
            );
          }
        });
      }
    }

    Ok(n)
  }
}
