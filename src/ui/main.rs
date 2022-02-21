use super::{
    states::{MainTitleState, UiState},
    ui::{
        extra_title_despawn, extra_title_spawn, main_title_despawn, main_title_spawn,
        title_load_images,
    },
};
use crate::system::buttons::ui_button_event;
use bevy::prelude::*;

// #[derive(Debug, Default, Clone, Hash, PartialEq, Eq, SystemLabel)]
// pub struct UiLabel(pub String);

// macro_rules! label {
//     ($s: expr) => {{
//         UiLabel($s.to_string())
//     }};
// }

#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(UiState::from(MainTitleState::Main))
            // main title ui
            .add_system_set(
                SystemSet::on_enter(UiState::from(MainTitleState::Main))
                    .with_system(main_title_spawn),
            )
            .add_system_set(
                SystemSet::on_exit(UiState::from(MainTitleState::Main))
                    .with_system(main_title_despawn),
            )
            // extra title ui
            .add_system_set(
                SystemSet::on_enter(UiState::from(MainTitleState::Extra))
                    .with_system(extra_title_spawn),
            )
            .add_system_set(
                SystemSet::on_exit(UiState::from(MainTitleState::Extra))
                    .with_system(extra_title_despawn),
            )
            .add_system_set(
                SystemSet::new()
                    .with_system(title_load_images)
                    .with_system(ui_button_event),
            );
    }
}
