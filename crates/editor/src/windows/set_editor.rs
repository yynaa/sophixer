use eframe::egui::DragValue;

use crate::windows::Window;

pub struct SetEditor {}

impl Default for SetEditor {
  fn default() -> Self {
    Self {}
  }
}

impl Window for SetEditor {
  fn title(&mut self) -> String {
    String::from("set editor")
  }

  fn ui(
    &mut self,
    model: &mut crate::Model,
    ui: &mut eframe::egui::Ui,
  ) -> anyhow::Result<Option<Box<dyn Window>>> {
    if let Some(set) = &mut model.set {
      {
        ui.horizontal(|ui| {
          ui.label("name");
          ui.text_edit_singleline(&mut set.name);
        });

        ui.horizontal(|ui| {
          ui.label("authors");
          ui.text_edit_singleline(&mut set.authors);
        });

        ui.horizontal(|ui| {
          ui.label("break sequence position");
          ui.add(DragValue::new(&mut set.stop_seq_pos));
        });
      }
    }

    Ok(None)
  }
}
