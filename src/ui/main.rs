use crate::{
    system::buttons::{game_exit_button_event, ui_button_event_curried},
    text,
    ui::{
        descriptors::{
            widget_descriptor_spawn, Descriptor, FontSettings, TextDescriptor, WidgetId,
        },
        resources::{
            UiImageResources, MAIN_TITLE_RES_MAP, START_TITLE_DIALOG_TEXTBOX_GUID,
            START_TITLE_NAME_TEXTBOX_GUID, START_TITLE_RES_MAP,
        },
        states::{
            MainTitleState, UiState, MAIN_TITLE_BUTTON_STATE_MAP, START_TITLE_BUTTON_STATE_MAP,
        },
        ui::{MAIN_TITLE_LAYOUT, START_TITLE_LAYOUT, WINDOW_HEIGHT},
    },
};
use bevy::{
    prelude::*,
    render::{camera::WindowOrigin, texture::DEFAULT_IMAGE_HANDLE},
};
use std::collections::HashMap;

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
                    .with_system(ui_button_event_curried(
                        &*MAIN_TITLE_BUTTON_STATE_MAP,
                        &*MAIN_TITLE_RES_MAP,
                    )),
            )
            .add_system_set(
                SystemSet::on_exit(UiState::from(MainTitleState::Main))
                    .with_system(title_despawn_curried(&*MAIN_TITLE_RES_MAP)),
            )
            // In-game ui
            .add_system_set(
                SystemSet::on_enter(UiState::from(MainTitleState::Start))
                    .with_system(title_spawn_curried(&*START_TITLE_LAYOUT))
                    .with_system(start_title_text_bundle_spawn.label("start_title_ui_system"))
                    .with_system(change_origin.after("start_title_ui_system")),
            )
            .add_system_set(
                SystemSet::on_update(UiState::from(MainTitleState::Start))
                    .with_system(title_load_images_curried(&*START_TITLE_RES_MAP))
                    .with_system(ui_button_event_curried(
                        &*START_TITLE_BUTTON_STATE_MAP,
                        &*START_TITLE_RES_MAP,
                    )),
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
            let mut command = commands.spawn(NodeBundle {
                ..Default::default()
            });
            // Generate a new entity and it's children by descriptor
            command.with_children(|title| {
                widget_descriptor_spawn(title, descriptor);
            });
        }
    }
}

// TODO: Text2dBundle CANNOT be added with UiBundle
fn start_title_text_bundle_spawn(mut commands: Commands) {
    commands
        .spawn(Text2dBundle::from(text!(
            (540.0, 90.0),
            (60.0, WINDOW_HEIGHT - 470.0),
            28.0,
            (255.0, 255.0, 255.0)
        )))
        .insert(START_TITLE_DIALOG_TEXTBOX_GUID);
    commands
        .spawn(Text2dBundle::from(text!(
            (125.0, 35.0),
            // (55.0, WINDOW_HEIGHT - 390.0),
            (127.5, WINDOW_HEIGHT - 407.5), // Center the text
            24.0,
            (255.0, 255.0, 255.0)
        )))
        .insert(START_TITLE_NAME_TEXTBOX_GUID);
}

fn change_origin(mut query: Query<&mut OrthographicProjection>) {
    for mut orth in query.iter_mut() {
        orth.window_origin = WindowOrigin::BottomLeft;
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
