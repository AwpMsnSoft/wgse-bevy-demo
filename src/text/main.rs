use crate::text::texts::{
    dialog_textbox_button_system, dialog_textbox_name_system, dialog_textbox_text_system,
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dialog_textbox_button_system)
            .add_system(dialog_textbox_name_system)
            .add_system(dialog_textbox_text_system);
    }

    fn name(&self) -> &str {
        "wgse_text"
    }
}
