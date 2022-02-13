use crate::ui::{
    resources::CONFIG_BUTTON_GUID,
    states::{MainTitleState, UIForStat, UIState},
    widget::WidgetID,
};
use bevy::prelude::*;

pub fn config_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
    mut ui_state: ResMut<State<UIState>>,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id.0 == CONFIG_BUTTON_GUID && ui_state.current() == &UIState::from(MainTitleState::Main) {
            match interaction {
                Interaction::Hovered => {
                    info!("ui/title Config button hovered.");
                    (*visibility).is_visible = true;
                }
                Interaction::None => {
                    (*visibility).is_visible = false;
                }
                Interaction::Clicked => {
                    ui_state.set(UIState::from(MainTitleState::Extra)).unwrap_or_else(|_| {
                        error!("Failed to set UIState::MainTitleState::Extra.");
                    });
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}
