use std::collections::HashMap;

use super::{
    descriptors::Descriptor,
    resources::{UiImageResources, MAIN_TITLE_RES_MAP, START_TITLE_RES_MAP},
    states::{MainTitleState, UiState, MAIN_TITLE_BUTTON_STATE_MAP, START_TITLE_BUTTON_STATE_MAP},
    ui::{MAIN_TITLE_LAYOUT, START_TITLE_LAYOUT},
};
use crate::{
    system::buttons::{game_exit_button_event, ui_button_event_curried},
    ui::descriptors::{widget_descriptor_spawn, WidgetId},
};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

#[derive(Debug)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(UiState::from(MainTitleState::Main))
            // main title ui
            .add_system_set(
                SystemSet::on_enter(UiState::from(MainTitleState::Main))
                    .with_system(title_spawn_curried(&*MAIN_TITLE_LAYOUT)),
            )
            .add_system_set(
                SystemSet::on_update(UiState::from(MainTitleState::Main))
                    .with_system(title_load_images_curried(&*MAIN_TITLE_RES_MAP))
                    .with_system(ui_button_event_curried(&*MAIN_TITLE_BUTTON_STATE_MAP)),
            )
            .add_system_set(
                SystemSet::on_exit(UiState::from(MainTitleState::Main))
                    .with_system(title_despawn_curried(&*MAIN_TITLE_RES_MAP)),
            )
            // In-game ui
            .add_system_set(
                SystemSet::on_enter(UiState::from(MainTitleState::Start))
                    .with_system(title_spawn_curried(&*START_TITLE_LAYOUT)),
            )
            .add_system_set(
                SystemSet::on_update(UiState::from(MainTitleState::Start))
                    .with_system(title_load_images_curried(&*START_TITLE_RES_MAP))
                    .with_system(ui_button_event_curried(&*START_TITLE_BUTTON_STATE_MAP)),
            )
            .add_system_set(
                SystemSet::on_exit(UiState::from(MainTitleState::Start))
                    .with_system(title_despawn_curried(&*START_TITLE_RES_MAP)),
            )
            .add_system_set(SystemSet::new().with_system(game_exit_button_event));
    }

    fn name(&self) -> &str {
        "wgse_ui"
    }
}

fn title_spawn_curried(layout: &'static Vec<Descriptor>) -> impl Fn(Commands, Res<State<UiState>>) {
    move |mut commands, ui_state| {
        debug!("Spawning title. current state: {:?}", ui_state);
        for descriptor in layout.iter() {
            // Logistic root node of this title
            let mut command = commands.spawn_bundle(NodeBundle {
                ..Default::default()
            });
            // Generate a new entity and it's children by descriptor
            command.with_children(|title| {
                widget_descriptor_spawn(title, descriptor);
            });
        }
    }
}

fn title_despawn_curried(
    res_map: &'static HashMap<WidgetId, &'static str>,
) -> impl Fn(Commands, Res<State<UiState>>, Query<(Entity, &WidgetId)>) {
    move |mut commands, ui_state, query| {
        debug!("Despawning title. current state: {:?}", ui_state.current());
        for (entity, widget_id) in query.iter() {
            if res_map.contains_key(&widget_id) {
                debug!("Despawning entity {:?}", entity);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn title_load_images_curried(
    res_map: &'static HashMap<WidgetId, &'static str>,
) -> impl Fn(Query<(&WidgetId, &mut UiImage)>, Res<UiImageResources>) {
    move |mut widgets_query, res| {
        for (id, mut image) in widgets_query.iter_mut() {
            if image.0 == DEFAULT_IMAGE_HANDLE.typed() && id != &WidgetId(0) {
                if let Some(res_name) = res_map.get(&id) {
                    debug!("Loading image for widget {:?}", id);
                    *image = res.0.get(res_name).unwrap().clone().into();
                }
            }
        }
    }
}
