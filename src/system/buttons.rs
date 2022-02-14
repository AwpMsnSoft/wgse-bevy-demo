use crate::ui::{
    resources::{
        BACK_BUTTON_GUID, CG_BUTTON_GUID, CONFIG_BUTTON_GUID, EXIT_BUTTON_GUID, EXTRA_BUTTON_GUID,
        MUSIC_BUTTON_GUID, SCENE_BUTTON_GUID, START_BUTTON_GUID,
    },
    states::{ExtraTitleState, MainTitleState, UIState},
    widget::WidgetID,
};
use bevy::{app::AppExit, prelude::*};

// main title ui
pub fn start_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    mut ui_state: ResMut<State<UIState>>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id == &START_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Main) {
            match interaction {
                Interaction::Hovered => {
                    debug!("ui/title Start button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    ui_state
                        .set(UIState::from(MainTitleState::Start))
                        .unwrap_or_else(|_| {
                            error!("Failed to set UIState::MainTitleState::Start.");
                        });
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}

pub fn config_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    mut ui_state: ResMut<State<UIState>>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id == &CONFIG_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Main) {
            match interaction {
                Interaction::Hovered => {
                    debug!("ui/title Config button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    ui_state
                        .set(UIState::from(MainTitleState::Config))
                        .unwrap_or_else(|_| {
                            error!("Failed to set UIState::MainTitleState::Config.");
                        });
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}

pub fn extra_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    mut ui_state: ResMut<State<UIState>>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id == &EXTRA_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Main) {
            match interaction {
                Interaction::Hovered => {
                    debug!("ui/title Extra button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    ui_state
                        .set(UIState::from(MainTitleState::Extra))
                        .unwrap_or_else(|_| {
                            error!("Failed to set UIState::MainTitleState::Extra.");
                        });
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}

pub fn exit_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    ui_state: Res<State<UIState>>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id == &EXIT_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Main) {
            match interaction {
                Interaction::Hovered => {
                    debug!("ui/title Exit button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    debug!("ui/title Exit button clicked.");
                    info!("Exiting...");
                    app_exit_event.send(AppExit);
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}

// extra title ui
pub fn cg_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    mut ui_state: ResMut<State<UIState>>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id == &CG_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Extra) {
            match interaction {
                Interaction::Hovered => {
                    debug!("ui/title Cg button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    ui_state
                        .set(UIState::from(ExtraTitleState::Cg))
                        .unwrap_or_else(|_| {
                            error!("Failed to set UIState::ExtraTitleState::Cg.");
                        });
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}

pub fn music_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    mut ui_state: ResMut<State<UIState>>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id == &MUSIC_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Extra) {
            match interaction {
                Interaction::Hovered => {
                    debug!("ui/title Music button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    ui_state
                        .set(UIState::from(ExtraTitleState::Music))
                        .unwrap_or_else(|_| {
                            error!("Failed to set UIState::ExtraTitleState::Music.");
                        });
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}

pub fn scene_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    mut ui_state: ResMut<State<UIState>>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id == &SCENE_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Extra) {
            match interaction {
                Interaction::Hovered => {
                    debug!("ui/title Scene button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    ui_state
                        .set(UIState::from(ExtraTitleState::Scene))
                        .unwrap_or_else(|_| {
                            error!("Failed to set UIState::ExtraTitleState::Scene.");
                        });
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}

pub fn back_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    mut ui_state: ResMut<State<UIState>>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id == &BACK_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Extra) {
            match interaction {
                Interaction::Hovered => {
                    debug!("ui/title Back button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    ui_state
                        .set(UIState::from(MainTitleState::Main))
                        .unwrap_or_else(|_| {
                            error!("Failed to set UIState::MainTitleState::Main.");
                        });
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}
