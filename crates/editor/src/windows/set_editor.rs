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
      }
    }

    Ok(None)
  }
}
