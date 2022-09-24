use crate::text::{
    states::TextRenderingState,
    texts::{
        dialog_textbox_button_system, dialog_textbox_name_system, dialog_textbox_text_system,
        dialog_textbox_text_trigger,
    },
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(TextRenderingState::Done)
            .add_system(dialog_textbox_button_system)
            .add_system(dialog_textbox_name_system)
            .add_system(dialog_textbox_text_trigger)
            .add_system(dialog_textbox_text_system);
    }

    fn name(&self) -> &str {
        "wgse_text"
    }
}
