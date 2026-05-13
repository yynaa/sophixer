use sophixer_core::song_data::Song;

use crate::windows::Window;

pub struct SongNew {
  new_id: String,
}

impl Default for SongNew {
  fn default() -> Self {
    Self {
      new_id: "new_song".to_string(),
    }
  }
}

impl Window for SongNew {
  fn title(&mut self) -> String {
    String::from("new song")
  }

  fn ui(
    &mut self,
    model: &mut crate::Model,
    ui: &mut eframe::egui::Ui,
  ) -> anyhow::Result<Option<Box<dyn Window>>> {
    if let Some(set) = &mut model.set {
      ui.text_edit_singleline(&mut self.new_id);
      if ui.button("create").clicked() {
        set.songs.insert(
          self.new_id.clone(),
          Song::new(self.new_id.clone(), String::new(), String::new()).unwrap(),
        );
      }
    }

    Ok(None)
  }
}
