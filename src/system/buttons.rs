use crate::ui::{
    resources::{
        BACK_BUTTON_GUID, CG_BUTTON_GUID, CONFIG_BUTTON_GUID, EXIT_BUTTON_GUID, EXTRA_BUTTON_GUID,
        MUSIC_BUTTON_GUID, SCENE_BUTTON_GUID, START_BUTTON_GUID,
    },
    states::{ExtraTitleState, MainTitleState, UiState},
    descriptors::WidgetId,
};
use bevy::{app::AppExit, prelude::*};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
struct UiButtonState {
    pub current: UiState,
    pub turning_into: Option<UiState>,
}

impl From<(UiState, Option<UiState>)> for UiButtonState {
    fn from(state: (UiState, Option<UiState>)) -> Self {
        UiButtonState {
            current: state.0,
            turning_into: state.1,
        }
    }
}

// This struct is used to store the state transition of the buttons.
lazy_static! {
    static ref UI_BUTTON_STATE_MAP: HashMap<WidgetId, UiButtonState> = {
        let mut map = HashMap::new();
        map.insert(
            START_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(MainTitleState::Start.into()),
            )),
        );
        map.insert(
            CONFIG_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(MainTitleState::Config.into()),
            )),
        );
        map.insert(
            EXTRA_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(MainTitleState::Extra.into()),
            )),
        );
        map.insert(
            EXIT_BUTTON_GUID,
            UiButtonState::from((MainTitleState::Main.into(), None)),
        );
        map.insert(
            MUSIC_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Music.into()),
            )),
        );
        map.insert(
            SCENE_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Scene.into()),
            )),
        );
        map.insert(
            CG_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Cg.into()),
            )),
        );
        map.insert(
            BACK_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(MainTitleState::Main.into()),
            )),
        );
        map
    };
}

// TODO: UI 界面的前两个按钮似乎失效了
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