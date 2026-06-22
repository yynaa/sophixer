use eframe::egui::{ComboBox, Ui};
use sophixer_core::data::channels::Channel;

pub fn channel_selector(value: &mut Channel, ui: &mut Ui) {
  ComboBox::from_label("channel")
    .selected_text(format!("{}", value))
    .show_ui(ui, |ui| {
      for i in 1..=6 {
        let sv = Channel::Lead(i);
        ui.selectable_value(value, sv.clone(), format!("{}", sv));
      }

      {
        let sv = Channel::MasterLead;
        ui.selectable_value(value, sv.clone(), format!("{}", sv));
      }

      for i in 1..=6 {
        let sv = Channel::Drum(i);
        ui.selectable_value(value, sv.clone(), format!("{}", sv));
      }

      {
        let sv = Channel::MasterDrum;
        ui.selectable_value(value, sv.clone(), format!("{}", sv));
      }

      {
        let sv = Channel::Master;
        ui.selectable_value(value, sv.clone(), format!("{}", sv));
      }
    });
}
