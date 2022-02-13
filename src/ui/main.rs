use super::{
    states::MainTitleState,
    ui::{extra_title_despawn, extra_title_spawn, main_title_despawn, main_title_spawn},
};
use crate::system::buttons::config_button_event;
use bevy::prelude::*;

#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MainTitleState::Main)
            .add_system_set(SystemSet::on_enter(MainTitleState::Main).with_system(main_title_spawn))
            .add_system_set(
                SystemSet::on_exit(MainTitleState::Main).with_system(main_title_despawn),
            )
            .add_system_set(
                SystemSet::on_enter(MainTitleState::Extra).with_system(extra_title_spawn),
            )
            .add_system_set(
                SystemSet::on_exit(MainTitleState::Extra).with_system(extra_title_despawn),
            )
            .add_system(config_button_event);
    }
}
