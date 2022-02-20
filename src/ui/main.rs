use super::{
    states::{MainTitleState, UIState},
    ui::{extra_title_despawn, extra_title_spawn, main_title_despawn, main_title_spawn, title_load_images},
};
use crate::system::buttons::{
    back_button_event, cg_button_event, config_button_event, exit_button_event, extra_button_event,
    music_button_event, scene_button_event, start_button_event,
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(UIState::from(MainTitleState::Main))
            // main title ui
            .add_system_set(
                SystemSet::on_enter(UIState::from(MainTitleState::Main))
                    .with_system(main_title_spawn)
            )
            .add_system_set(
                SystemSet::on_update(UIState::from(MainTitleState::Main))
                    .with_system(title_load_images)
            )
            .add_system_set(
                SystemSet::on_exit(UIState::from(MainTitleState::Main))
                    .with_system(main_title_despawn),
            )
            .add_system_set(
                SystemSet::new()
                    .with_system(start_button_event)
                    .with_system(config_button_event)
                    .with_system(extra_button_event)
                    .with_system(exit_button_event),
            )
            // extra title ui
            .add_system_set(
                SystemSet::on_enter(UIState::from(MainTitleState::Extra))
                    .with_system(extra_title_spawn)
            )
            .add_system_set(
                SystemSet::on_update(UIState::from(MainTitleState::Extra))
                    .with_system(title_load_images),
            )
            .add_system_set(
                SystemSet::on_exit(UIState::from(MainTitleState::Extra))
                    .with_system(extra_title_despawn),
            )
            .add_system_set(
                SystemSet::new()
                    .with_system(scene_button_event)
                    .with_system(music_button_event)
                    .with_system(cg_button_event)
                    .with_system(back_button_event),
            );
    }
}
