use eframe::egui::{self};
use sophixer_core::song_data::Set;

use crate::windows::{set_editor::SetEditor, song_editor::SongEditor, song_new::SongNew, Window};

pub mod windows;

fn main() -> eframe::Result {
  pretty_env_logger::init();

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1280., 720.]),
    ..Default::default()
  };

  eframe::run_native(
    "Sophixeditor",
    options,
    Box::new(|_cc| {
      // This gives us image support:
      // egui_extras::install_image_loaders(&cc.egui_ctx);

      Ok(Box::<App>::default())
    }),
  )
}

struct App {
  model: Model,
  set_folder: Option<String>,
  windows: Vec<(Box<dyn Window>, bool)>,
}

pub struct Model {
  set: Option<Set>,
}

impl Default for Model {
  fn default() -> Self {
    Self {
      // set: None
      set: Some(Set::from_folder("/code/p/sophixer/sets/10".to_string()).unwrap()),
    }
  }
}

impl Default for App {
  fn default() -> Self {
    Self {
      model: Model::default(),
      // set_folder: None,
      set_folder: Some("/code/p/sophixer/sets/10".to_string()),
      windows: Vec::new(),
    }
  }
}

impl eframe::App for App {
  fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
      if ui.button("Open set folder…").clicked() {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
          self.model.set = Some(Set::from_folder(path.to_string_lossy().to_string()).unwrap());
          self.set_folder = Some(path.to_string_lossy().to_string());
        }
      }

      if let Some(set_folder) = &self.set_folder {
        if let Some(set) = &mut self.model.set {
          if ui.button("Save set").clicked() {
            set.save_in_folder(set_folder.clone()).unwrap();
          }

          if ui.button("Set Editor").clicked() {
            self.windows.push((Box::new(SetEditor::default()), true));
          }

          ui.label("songs:");
          for (song_id, song) in &set.songs {
            if ui.button(song_id.clone()).clicked() {
              self
                .windows
                .push((Box::new(SongEditor::new(song_id.clone(), song)), true));
            }
          }
          if ui.button("..new").clicked() {
            self.windows.push((Box::new(SongNew::default()), true));
          }
        }
      }

      let mut to_be_added = Vec::new();
      for window in &mut self.windows {
        if window.1 {
          let n = window.0.show(&mut self.model, ui, &mut window.1);
          if let Some(n) = n {
            to_be_added.push(n);
          }
        }
      }
      for t in to_be_added {
        self.windows.push((t, true));
      }

      for i in (0..self.windows.len()).rev() {
        let (_, b) = self.windows.get(i).unwrap();
        if !b {
          self.windows.remove(i);
        }
      }

      // yeah
    });
  }
}
