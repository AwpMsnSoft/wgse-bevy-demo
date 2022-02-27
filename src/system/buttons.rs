use crate::ui::{
    descriptors::WidgetId,
    resources::EXIT_BUTTON_GUID,
    states::{UiButtonState, UiState, TITLE_BUTTON_STATE_LIST},
};
use bevy::{app::AppExit, prelude::*};
use lazy_static::lazy_static;
use std::collections::HashMap;

// This struct is used to store the state transition of the buttons.
lazy_static! {
    static ref UI_BUTTON_STATE_MAP: HashMap<WidgetId, UiButtonState> = {
        let mut map = HashMap::new();
        for list in TITLE_BUTTON_STATE_LIST.iter() {
            for (widget_id, state) in list.iter() {
                map.insert(widget_id.clone(), state.clone());
            }
        }
        map
    };
}

pub fn ui_button_event(
    mut interaction_query: Query<(&Interaction, &mut Visibility, &WidgetId), Changed<Interaction>>,
    mut ui_state: ResMut<State<UiState>>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for (interaction, mut visibility, widget_id) in interaction_query.iter_mut() {
        if let Some(button_state) = UI_BUTTON_STATE_MAP.get(widget_id) {
            if &button_state.current == ui_state.current() {
                match interaction {
                    Interaction::Hovered => {
                        debug!("Button {:?} is hovered", widget_id);
                        visibility.is_visible = true;
                    }
                    Interaction::None => {
                        visibility.is_visible = false;
                    }
                    Interaction::Clicked => {
                        if let Some(next_state) = &button_state.turning_into {
                            ui_state.set(next_state.clone()).unwrap_or_else(|err| {
                                panic!(
                                    "Failed to set ui state to {:?}, error is {:?}",
                                    next_state, err
                                )
                            });
                        }
                        // Exit button on main title screen, exit the app.
                        if widget_id == &EXIT_BUTTON_GUID {
                            info!("Exiting the game...");
                            app_exit_event.send(AppExit);
                        }
                    }
                }
            }
        }
    }
}
