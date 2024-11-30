pub mod variants;
use std::sync::Arc;
pub use variants::*;

pub fn add_to_fonts(fonts: &mut egui::FontDefinitions, variant: Variant) {
  fonts.font_data.insert("phosphor".into(), Arc::new(variant.font_data()));

  if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
    font_keys.insert(1, "phosphor".into());
  }
}
