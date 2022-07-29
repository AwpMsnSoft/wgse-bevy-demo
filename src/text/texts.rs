use crate::{
    script::commands as wgs,
    ui::{
        descriptors::{Descriptor, TextDescriptor, WidgetId},
        resources::{
            FontResources, START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID,
            START_TITLE_DIALOG_TEXTBOX_GUID, START_TITLE_NAME_TEXTBOX_GUID,
        },
        states::{MainTitleState, UiState},
    },
};
use bevy::prelude::*;

pub fn dialog_textbox_button_system(
    mut interaction_query: Query<(&Interaction, &WidgetId), Changed<Interaction>>,
    ui_state: Res<State<UiState>>,
    mut next: EventWriter<wgs::Next>,
) {
    for (interaction, widget_id) in interaction_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID
        {
            if interaction == &Interaction::Clicked {
                // Get next dialog from script
                debug!("Getting next dialog from script...");
                next.send(wgs::Next {});
            }
        }
    }
}

pub fn dialog_textbox_name_system(
    mut text_query: Query<(&mut Text, &WidgetId), With<Parent>>,
    ui_state: Res<State<UiState>>,
    font: Res<FontResources>,
    mut command: EventReader<wgs::Message>,
) {
    for (mut text, widget_id) in text_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_NAME_TEXTBOX_GUID
        {
            if let Some(message) = command.iter().last() {
                info!("Getting name from script, message: {}", message.chara);
                text.sections[0].value = message.chara.clone();
                text.sections[0].style.font = font.0.clone();
            }
        }
    }
}

pub fn dialog_textbox_text_system(
    mut text_query: Query<(&mut Text, &Style, &WidgetId), With<Parent>>,
    ui_state: Res<State<UiState>>,
    font: Res<FontResources>,
    mut command: EventReader<wgs::Message>,
) {
    for (mut text, style, widget_id) in text_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_DIALOG_TEXTBOX_GUID
        {
            if let Some(message) = command.iter().last() {
                info!("Getting dialog from script, message: {}", message.message);
                text.sections[0].value = message.message.clone();
                text.sections[0].style.font = font.0.clone();
                info!("Text: {:?}", (text, style));
            }
        }
    }
}
