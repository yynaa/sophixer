use anyhow::Result;

use crate::Model;

pub mod button_editor;
pub mod section_editor;
pub mod set_editor;
pub mod song_editor;
pub mod song_new;

pub trait Window {
  fn title(&mut self) -> String;

  fn ui(&mut self, model: &mut Model, ui: &mut eframe::egui::Ui)
    -> Result<Option<Box<dyn Window>>>;

  fn show(
    &mut self,
    model: &mut Model,
    ui: &mut eframe::egui::Ui,
    open: &mut bool,
  ) -> Option<Box<dyn Window>> {
    let window = eframe::egui::Window::new(self.title())
      .open(open)
      .resizable(false);

    let new = window.show(ui, |ui| self.ui(model, ui).unwrap());
    if let Some(new) = new {
      if let Some(new) = new.inner {
        return new;
      }
    }

    None
  }
}
