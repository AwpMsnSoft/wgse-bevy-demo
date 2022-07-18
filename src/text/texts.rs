use crate::ui::{
    descriptors::{Descriptor, TextDescriptor, WidgetId},
    resources::{
        FontResources, START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID,
        START_TITLE_DIALOG_TEXTBOX_GUID, START_TITLE_NAME_TEXTBOX_GUID,
    },
    states::{MainTitleState, StartTitleState, UiState},
};
use bevy::prelude::*;

pub struct Message(pub ());
pub struct MessageContent(pub String);
pub struct MessageName(pub String);

pub fn dialog_textbox_button_system(
    mut interaction_query: Query<(&Interaction, &WidgetId), (With<Children>, Changed<Interaction>)>,
    ui_state: Res<State<UiState>>,
    mut dialog_fetch_event: EventWriter<Message>,
) {
    for (interaction, widget_id) in interaction_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID
        {
            if interaction == &Interaction::Clicked {
                // Get next dialog from script
                dialog_fetch_event.send(Message(()));
            }
        }
    }
}

pub fn dialog_textbox_name_system(
    mut text_query: Query<(&mut Text, &WidgetId), With<Children>>,
    ui_state: Res<State<UiState>>,
    mut dialog_name_event: EventReader<MessageName>,
) {
    for (mut text, widget_id) in text_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_NAME_TEXTBOX_GUID
        {
            if let Some(dialog_name) = dialog_name_event.iter().last() {
                text.sections[0].value = dialog_name.0.clone();
            }
        }
    }
}

pub fn dialog_textbox_text_system(
    mut text_query: Query<(&mut Text, &WidgetId), With<Parent>>,
    ui_state: Res<State<UiState>>,
    mut dialog_content_event: EventReader<MessageContent>,
) {
    for (mut text, widget_id) in text_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_DIALOG_TEXTBOX_GUID
        {
            if let Some(MessageContent(content)) = dialog_content_event.iter().last() {
                text.sections[0].value = content.clone();
            }
        }
    }
}

// pub fn game_exit_button_event(
//     mut interaction_query: Query<(&Interaction, &WidgetId), Changed<Interaction>>,
//     ui_state: ResMut<State<UiState>>,
//     mut app_exit_event: EventWriter<AppExit>,
// ) {
//     for (interaction, widget_id) in interaction_query.iter_mut() {
//         if ui_state.current() == &UiState::from(MainTitleState::Main)
//             && widget_id == &MAIN_TITLE_EXIT_BUTTON_GUID
//         {
//             if interaction == &Interaction::Clicked {
//                 info!("Exiting the game...");
//                 app_exit_event.send(AppExit);
//             }
//         }
//     }
// }
