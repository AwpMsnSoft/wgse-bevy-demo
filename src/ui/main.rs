use std::collections::HashMap;

use super::{
    descriptors::Descriptor,
    resources::{UiImageResources, EXTRA_TITLE_RES_MAP, MAIN_TITLE_RES_MAP},
    states::{MainTitleState, UiState},
    ui::{EXTRA_TITLE_LAYOUT, MAIN_TITLE_LAYOUT},
};
use crate::{
    system::buttons::ui_button_event,
    ui::descriptors::{widget_descriptor_spawn, WidgetBundle, WidgetDescriptor, WidgetId},
};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

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
                    .with_system(title_spawn_curried(&*MAIN_TITLE_LAYOUT)),
            )
            .add_system_set(
                SystemSet::on_update(UiState::from(MainTitleState::Main))
                    .with_system(title_load_images_curried(&*MAIN_TITLE_RES_MAP)),
            )
            .add_system_set(
                SystemSet::on_exit(UiState::from(MainTitleState::Main))
                    .with_system(title_despawn_curried(&*MAIN_TITLE_RES_MAP)),
            )
            // extra title ui
            .add_system_set(
                SystemSet::on_enter(UiState::from(MainTitleState::Extra))
                    .with_system(title_spawn_curried(&*EXTRA_TITLE_LAYOUT)),
            )
            .add_system_set(
                SystemSet::on_update(UiState::from(MainTitleState::Extra))
                    .with_system(title_load_images_curried(&*EXTRA_TITLE_RES_MAP)),
            )
            .add_system_set(
                SystemSet::on_exit(UiState::from(MainTitleState::Extra))
                    .with_system(title_despawn_curried(&*EXTRA_TITLE_RES_MAP)),
            )
            .add_system_set(SystemSet::new().with_system(ui_button_event));
    }
}

pub fn title_spawn_curried(
    layout: &'static Vec<Descriptor>,
) -> impl Fn(Commands, Res<State<UiState>>) {
    move |mut commands, ui_state| {
        debug!("Spawning title. current state: {:?}", ui_state);
        for descriptor in layout.iter() {
            // Logistic root node of this title
            let mut command = commands.spawn_bundle(NodeBundle {
                ..Default::default()
            });
            // Generate a new entity by descriptor
            command.with_children(|title| {
                widget_descriptor_spawn(title, descriptor);
            });
            // Generate all sub-widgets by descriptor.children
            if let Some(children) = &descriptor.children {
                command.with_children(|parent| {
                    for child in &children.0 {
                        widget_descriptor_spawn(parent, child);
                    }
                });
            }
        }
    }
}

pub fn title_despawn_curried(
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

pub fn title_load_images_curried(
    res_map: &'static HashMap<WidgetId, &'static str>,
) -> impl Fn(Query<(&WidgetId, &mut UiImage)>, Res<UiImageResources>) {
    move |mut widgets_query, res| {
        for (id, mut image) in widgets_query.iter_mut() {
            if image.0 == DEFAULT_IMAGE_HANDLE.typed() && id != &WidgetId(0) {
                debug!("Loading image for widget {:?}", id);
                *image = res.0.get(res_map.get(&id).unwrap()).unwrap().clone().into();
            }
        }
    }
}
