use crate::ui::{
    descriptors::{Descriptor, TextDescriptor, WidgetId},
    resources::{FontResources, START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID},
    states::{StartTitleState, UiState, MainTitleState},
};
use bevy::prelude::*;

pub fn dialog_textbox_button_system(
    mut interaction_query: Query<(&Interaction, &WidgetId, &Children), Changed<Interaction>>,
    mut text_query: Query<&Text>,
    ui_state: Res<State<UiState>>
) {
    for (interaction, widget_id, children) in interaction_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start) &&
            widget_id == &START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID {
                // TODO: Get next dialog from script

                // TODO: Change text to next dialog
                let mut text = text_query.get_mut(children[0]).unwrap();
                // do something with text
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