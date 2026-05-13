use eframe::egui::{color_picker::color_edit_button_srgb, DragValue};
use sophixer_core::song_data::{Song, SongSection};

use crate::windows::{section_editor::SectionEditor, Window};

pub struct SongEditor {
  song_id: String,
  new_section: i64,
}

impl SongEditor {
  pub fn new(song_id: String, _song: &Song) -> Self {
    Self {
      song_id,
      new_section: 1,
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
          ui.label("path");
          ui.text_edit_singleline(&mut song.path);
        });

        ui.horizontal(|ui| {
          ui.label("order");
          ui.add(DragValue::new(&mut song.order));
        });

        ui.horizontal(|ui| {
          ui.label("bpm");
          ui.add(DragValue::new(&mut song.bpm));
        });

        ui.horizontal(|ui| {
          ui.label("color");
          color_edit_button_srgb(ui, &mut song.color);
        });

        ui.heading("sections");

        let mut sorted_sections = song.sections.keys().map(|f| *f).collect::<Vec<i64>>();
        sorted_sections.sort();

        for secid in sorted_sections {
          if ui.button(format!("section @ line {}", secid)).clicked() {
            if let Some(section) = song.sections.get(&secid) {
              n = Some(Box::new(SectionEditor::new(
                self.song_id.clone(),
                secid,
                section,
              )));
            }
          }
        }

        ui.horizontal(|ui| {
          ui.add(DragValue::new(&mut self.new_section));
          if ui.button("..new").clicked() {
            song
              .sections
              .insert(self.new_section, SongSection::default());
          }
        });
      }
    }

    Ok(n)
  }
}
