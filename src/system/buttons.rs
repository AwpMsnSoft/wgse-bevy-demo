use crate::ui::{
    descriptors::WidgetId,
    resources::{MAIN_TITLE_EXIT_BUTTON_GUID, UiImageResources},
    states::{MainTitleState, UiButtonState, UiState},
};
use bevy::{app::AppExit, prelude::*};
use std::collections::HashMap;

pub fn ui_button_event_curried(
    state_map: &'static HashMap<WidgetId, UiButtonState>,
    res_map: &'static HashMap<WidgetId, &'static str>,
) -> impl Fn(
    Query<(&Interaction, &mut UiImage, &WidgetId), Changed<Interaction>>,
    ResMut<State<UiState>>,
    Res<UiImageResources>
) {
    move |mut interaction_query, mut ui_state, ui_images| {
        for (interaction, mut image,  widget_id) in interaction_query.iter_mut() {
            if let Some(button_state) = state_map.get(widget_id) {
                if &button_state.current == ui_state.current() {
                    match interaction {
                        Interaction::Hovered => {
                            debug!("Button {:?} is hovered", widget_id);
                            if let Some(image_name) = res_map.get(&widget_id) {
                                *image = ui_images.0.get(image_name).unwrap().clone().into();
                            }
                        }
                        Interaction::None => {
                            // TODO: Forcibly requires "default.png"
                            *image = ui_images.0.get("default.png").unwrap().clone_weak().into();
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
                        }
                    }
                }
            }
        }
    }
}

pub fn game_exit_button_event(
    mut interaction_query: Query<(&Interaction, &WidgetId), Changed<Interaction>>,
    ui_state: ResMut<State<UiState>>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for (interaction, widget_id) in interaction_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Main)
            && widget_id == &MAIN_TITLE_EXIT_BUTTON_GUID
        {
            if interaction == &Interaction::Clicked {
                info!("Exiting the game...");
                app_exit_event.send(AppExit);
            }
        }
    }
}
