use egui::WidgetText;

pub fn kv_row(ui: &mut egui::Ui, kv: (impl Into<WidgetText>, impl Into<WidgetText>)) {
  ui.label(kv.0);
  ui.label(kv.1);
  ui.end_row();
}
