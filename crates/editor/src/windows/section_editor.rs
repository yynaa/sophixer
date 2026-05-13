use eframe::egui::{color_picker::color_edit_button_srgb, DragValue};
use sophixer_core::song_data::{SongButton, SongButtonAction, SongSection};

use crate::windows::{button_editor::ButtonEditor, Window};

pub struct SectionEditor {
  song_id: String,
  section_id: i64,
  new_button: i64,
}

impl SectionEditor {
  pub fn new(song_id: String, section_id: i64, _section: &SongSection) -> Self {
    Self {
      song_id,
      section_id,
      new_button: 1,
    }
  }
}
impl Window for SectionEditor {
  fn title(&mut self) -> String {
    format!("section editor: {}@{}", self.song_id, self.section_id)
  }

  fn ui(
    &mut self,
    model: &mut crate::Model,
    ui: &mut eframe::egui::Ui,
  ) -> anyhow::Result<Option<Box<dyn Window>>> {
    let mut n: Option<Box<dyn Window>> = None;

    if let Some(set) = &mut model.set {
      if let Some(song) = set.songs.get_mut(&self.song_id) {
        ui.heading("manage");

        if ui.button("delete").clicked() {
          song.sections.remove(&self.section_id);
        }

        if let Some(section) = song.sections.get_mut(&self.section_id) {
          ui.heading("info");

          ui.horizontal(|ui| {
            ui.label("start");
            ui.add(DragValue::new(&mut section.start));
          });
          ui.horizontal(|ui| {
            ui.label("length");
            ui.add(DragValue::new(&mut section.length));
          });
          ui.horizontal(|ui| {
            ui.label("color");
            color_edit_button_srgb(ui, &mut section.color);
          });

          ui.heading("buttons");

          let mut sorted_buttons = section.buttons.keys().map(|f| *f).collect::<Vec<i64>>();
          sorted_buttons.sort();

          for button_id in sorted_buttons {
            if ui.button(format!("button @ col {}", button_id)).clicked() {
              if let Some(button) = section.buttons.get(&button_id) {
                n = Some(Box::new(ButtonEditor::new(
                  self.song_id.clone(),
                  self.section_id,
                  button_id,
                  button,
                )));
              }
            }
          }

          ui.horizontal(|ui| {
            ui.add(DragValue::new(&mut self.new_button));
            if ui.button("..new").clicked() {
              section.buttons.insert(
                self.new_button,
                SongButton::new(SongButtonAction::default_toggle_channels().unwrap()).unwrap(),
              );
            }
          });
        }
      }
    }

    Ok(n)
  }
}
