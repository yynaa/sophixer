#[macro_use]
extern crate log;

use std::{
  fs::{File, read_to_string},
  io::Write,
};

use eframe::egui::{self};
use sophixer_core::data::{Set, Song};
use tin_drivers_midi::{MidiDriver, devices::launchpad_mini_mk3::LPM3Driver};

use crate::windows::{Window, set_editor::SetEditor, song_editor::SongEditor, song_new::SongNew};

pub mod widgets;
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
  set_file: Option<String>,
  windows: Vec<(Box<dyn Window>, bool)>,
}

pub struct Model {
  set: Option<Set>,
  lpm3driver: Option<LPM3Driver>,
}

impl Default for Model {
  fn default() -> Self {
    Self {
      set: None,
      lpm3driver: None,
    }
  }
}

impl Default for App {
  fn default() -> Self {
    Self {
      model: Model::default(),
      set_file: None,
      windows: Vec::new(),
    }
  }
}

impl eframe::App for App {
  fn on_exit(&mut self) {
    if let Some(driver) = &mut self.model.lpm3driver {
      driver.close().unwrap();
    }
  }

  fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
      if let Some(driver) = &mut self.model.lpm3driver {
        if ui.button("Close LPM3").clicked() {
          driver.close().unwrap();
          self.model.lpm3driver = None;
        }
      } else {
        if ui.button("Connect LPM3").clicked() {
          self.model.lpm3driver = Some(LPM3Driver::connect().unwrap());
        }
      }

      if ui.button("New set").clicked() {
        if let Some(path) = rfd::FileDialog::new().save_file() {
          self.model.set = Some(Set::new(String::new(), String::new(), String::new()).unwrap());
          self.set_file = Some(path.to_string_lossy().to_string());
        }
      }

      if ui.button("Open set…").clicked() {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
          if let Ok(set_string) = read_to_string(path.clone()) {
            self.model.set = Some(ron::from_str(&set_string).unwrap());
            self.set_file = Some(path.to_string_lossy().to_string());
          } else {
            error!("couldn't load set")
          }
        }
      }

      if let Some(set_file) = &self.set_file {
        if let Some(set) = &mut self.model.set {
          if ui.button("Save set").clicked() {
            if let Ok(mut file) = File::create(set_file) {
              file
                .write_all(&ron::to_string(set).unwrap().into_bytes())
                .unwrap();
            } else {
              error!("couldn't save set in file")
            }
          }

          if ui.button("Set Editor").clicked() {
            self.windows.push((Box::new(SetEditor::default()), true));
          }

          ui.label("songs:");

          let mut songs_sorted = set
            .songs
            .iter()
            .map(|(id, s)| (id.clone(), s.order, s))
            .collect::<Vec<(String, i64, &Song)>>();
          songs_sorted.sort_by(|a, b| a.1.cmp(&b.1));

          for (song_id, _order, song) in &songs_sorted {
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
