use super::{descriptors::WidgetID, resources::UiImageResources, states::UiState};
use crate::{
    button, image,
    ui::{
        descriptors::{ButtonDescriptor, ImageDescriptor, WidgetBundle},
        resources::{
            BACK_BUTTON_GUID, CG_BUTTON_GUID, CONFIG_BUTTON_GUID, EXIT_BUTTON_GUID,
            EXTRA_BUTTON_GUID, EXTRA_TILTE_BG_IAMGE, EXTRA_TITLE_BG_GUID, MAIN_TITLE_BG_GUID,
            MAIN_TITLE_BG_IMAGE, MUSIC_BUTTON_GUID, SCENE_BUTTON_GUID, START_BUTTON_GUID,
        },
    },
};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

pub fn main_title_spawn(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    debug!("Enter main_title_spawn.");
    let windows = windows.get_primary().unwrap();
    commands
        // main title background
        .spawn_bundle(WidgetBundle {
            id: MAIN_TITLE_BG_GUID,
            children: image! {
                windows.width(), windows.height(),
                0.0, windows.height(),
                MAIN_TITLE_BG_IMAGE
            }
            .load(&asset_server),
        })
        .with_children(|title| {
            // 1. start button
            title.spawn_bundle(WidgetBundle {
                id: START_BUTTON_GUID,
                children: button! {
                    100.0, 40.0,
                    200.0, windows.height() - 480.0
                },
            });
            // 2. config button
            title.spawn_bundle(WidgetBundle {
                id: CONFIG_BUTTON_GUID,
                children: button! {
                    100.0, 40.0,
                    300.0, windows.height() - 480.0
                },
            });
            // 3. extra button
            title.spawn_bundle(WidgetBundle {
                id: EXTRA_BUTTON_GUID,
                children: button! {
                    100.0, 40.0,
                    400.0, windows.height() - 480.0
                },
            });
            // 4. exit button
            title.spawn_bundle(WidgetBundle {
                id: EXIT_BUTTON_GUID,
                children: button! {
                    100.0, 40.0,
                    500.0, windows.height() - 480.0
                },
            });
        });
}

pub fn main_title_despawn(
    mut commands: Commands,
    state: Res<State<UiState>>,
    query: Query<(Entity, &WidgetID, &Children)>,
) {
    for (entity, id, children) in &mut query.iter() {
        debug!("Enter main_title_despawn. current state: {:?}", state);
        if id == &MAIN_TITLE_BG_GUID {
            debug!("main_title_despawn: entity: {:?}", entity);
            for &child in children.iter() {
                debug!("main_title_despawn: child: {:?}", child);
                commands.entity(child).despawn();
            }
            commands.entity(entity).despawn();
        }
    }
}

pub fn extra_title_spawn(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    debug!("Enter extra_title_spawn.");
    let windows = windows.get_primary().unwrap();
    commands
        // extra title background
        .spawn_bundle(WidgetBundle {
            id: EXTRA_TITLE_BG_GUID,
            children: image! {
                windows.width(), windows.height(),
                0.0, windows.height(),
                EXTRA_TILTE_BG_IAMGE
            }
            .load(&asset_server),
        })
        .with_children(|title| {
            // 1. cg button
            title.spawn_bundle(WidgetBundle {
                id: CG_BUTTON_GUID,
                children: button! {
                    100.0, 40.0,
                    200.0, windows.height() - 480.0
                },
            });
            // 2. scene button
            title.spawn_bundle(WidgetBundle {
                id: SCENE_BUTTON_GUID,
                children: button! {
                    100.0, 40.0,
                    300.0, windows.height() - 480.0
                },
            });
            // 3. music button
            title.spawn_bundle(WidgetBundle {
                id: MUSIC_BUTTON_GUID,
                children: button! {
                    100.0, 40.0,
                    400.0, windows.height() - 480.0
                },
            });
            // 4. back button
            title.spawn_bundle(WidgetBundle {
                id: BACK_BUTTON_GUID,
                children: button! {
                    100.0, 40.0,
                    500.0, windows.height() - 480.0
                },
            });
        });
}

pub fn extra_title_despawn(
    mut commands: Commands,
    state: Res<State<UiState>>,
    query: Query<(Entity, &WidgetID, &Children)>,
) {
    for (entity, id, children) in &mut query.iter() {
        debug!("Enter extra_title_despawn. current state: {:?}", state);
        if id == &EXTRA_TITLE_BG_GUID {
            debug!("extra_title_despawn: entity: {:?}", entity);
            for &child in children.iter() {
                debug!("extra_title_despawn: child: {:?}", child);
                commands.entity(child).despawn();
            }
            commands.entity(entity).despawn();
        }
    }
}

pub fn title_load_images(
    mut widgets_query: Query<(&WidgetID, &mut UiImage)>,
    res: Res<UiImageResources>,
) {
    for (id, mut image) in widgets_query.iter_mut() {
        if image.0 == DEFAULT_IMAGE_HANDLE.typed() {
            debug!("loading image for widget: {:?}", id);
            *image = res.0.get(id).unwrap().clone().into();
        }
    }
}
