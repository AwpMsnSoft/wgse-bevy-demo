use super::{resources::UiImageResources, states::UIState, widget::WidgetID};
use crate::ui::{
    resources::{
        BACK_BUTTON_GUID, CG_BUTTON_GUID, CONFIG_BUTTON_GUID, EXIT_BUTTON_GUID, EXTRA_BUTTON_GUID,
        EXTRA_TITLE_BG_GUID, MAIN_TITLE_BG_GUID, MUSIC_BUTTON_GUID, SCENE_BUTTON_GUID,
        START_BUTTON_GUID,
    },
    states::{MainTitleState, UIForStat},
    widget::WidgetBundle,
};
use bevy::prelude::*;
use std::sync::Once;

pub fn main_title_spawn(
    mut commands: Commands,
    windows: Res<Windows>,
    // main_title_res: Res<MainTitleRes>,
) {
    debug!("Enter main_title_spawn.");
    let windows = windows.get_primary().unwrap();
    commands
        // main title background
        .spawn_bundle(WidgetBundle {
            id: MAIN_TITLE_BG_GUID,
            children: ImageBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(windows.width() as f32),
                        height: Val::Px(windows.height() as f32),
                    },
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                // image: main_title_res.bg_image.clone().into(),
                ..Default::default()
            },
        })
        .with_children(|title| {
            // 1. start button
            title.spawn_bundle(WidgetBundle {
                id: START_BUTTON_GUID,
                children: ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                        },
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(200.0),
                            right: Val::Px(300.0),
                            top: Val::Px(windows.height() - 480.0),
                            bottom: Val::Px(windows.height() - 520.0),
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: false },
                    // image: main_title_res.start_button_hover_image.clone().into(),
                    ..Default::default()
                },
            });
            // 2. config button
            title.spawn_bundle(WidgetBundle {
                id: CONFIG_BUTTON_GUID,
                children: ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                        },
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(300.0),
                            right: Val::Px(400.0),
                            top: Val::Px(windows.height() - 480.0),
                            bottom: Val::Px(windows.height() - 520.0),
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: false },
                    // image: main_title_res.config_button_hover_image.clone().into(),
                    ..Default::default()
                },
            });
            // 3. extra button
            title.spawn_bundle(WidgetBundle {
                id: EXTRA_BUTTON_GUID,
                children: ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                        },
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(400.0),
                            right: Val::Px(500.0),
                            top: Val::Px(windows.height() - 480.0),
                            bottom: Val::Px(windows.height() - 520.0),
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: false },
                    // image: main_title_res.extra_button_hover_image.clone().into(),
                    ..Default::default()
                },
            });
            // 4. exit button
            title.spawn_bundle(WidgetBundle {
                id: EXIT_BUTTON_GUID,
                children: ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                        },
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(500.0),
                            right: Val::Px(600.0),
                            top: Val::Px(windows.height() - 480.0),
                            bottom: Val::Px(windows.height() - 520.0),
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: false },
                    // image: main_title_res.exit_button_hover_image.clone().into(),
                    ..Default::default()
                },
            });
        });
}

pub fn main_title_despawn(
    mut commands: Commands,
    state: Res<State<UIState>>,
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
    // extra_title_res: Res<ExtraTitleRes>,
) {
    debug!("Enter extra_title_spawn.");
    let windows = windows.get_primary().unwrap();
    commands
        // extra title background
        .spawn_bundle(WidgetBundle {
            id: EXTRA_TITLE_BG_GUID,
            children: ImageBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(windows.width() as f32),
                        height: Val::Px(windows.height() as f32),
                    },
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                // image: extra_title_res.bg_image.clone().into(),
                ..Default::default()
            },
        })
        .with_children(|title| {
            // 1. cg button
            title.spawn_bundle(WidgetBundle {
                id: CG_BUTTON_GUID,
                children: ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                        },
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(200.0),
                            right: Val::Px(300.0),
                            top: Val::Px(windows.height() - 480.0),
                            bottom: Val::Px(windows.height() - 520.0),
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: false },
                    // image: extra_title_res.cg_button_hover_image.clone().into(),
                    ..Default::default()
                },
            });
            // 2. scene button
            title.spawn_bundle(WidgetBundle {
                id: SCENE_BUTTON_GUID,
                children: ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                        },
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(300.0),
                            right: Val::Px(400.0),
                            top: Val::Px(windows.height() - 480.0),
                            bottom: Val::Px(windows.height() - 520.0),
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: false },
                    // image: extra_title_res.scene_button_hover_image.clone().into(),
                    ..Default::default()
                },
            });
            // 3. music button
            title.spawn_bundle(WidgetBundle {
                id: MUSIC_BUTTON_GUID,
                children: ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                        },
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(400.0),
                            right: Val::Px(500.0),
                            top: Val::Px(windows.height() - 480.0),
                            bottom: Val::Px(windows.height() - 520.0),
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: false },
                    // image: extra_title_res.music_button_hover_image.clone().into(),
                    ..Default::default()
                },
            });
            // 4. back button
            title.spawn_bundle(WidgetBundle {
                id: BACK_BUTTON_GUID,
                children: ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                        },
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(500.0),
                            right: Val::Px(600.0),
                            top: Val::Px(windows.height() - 480.0),
                            bottom: Val::Px(windows.height() - 520.0),
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: false },
                    // image: extra_title_res.back_button_hover_image.clone().into(),
                    ..Default::default()
                },
            });
        });
}

pub fn extra_title_despawn(
    mut commands: Commands,
    state: Res<State<UIState>>,
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
    mut widgets_query: Query<(&WidgetID, &mut UiImage, &mut Visibility)>,
    res: Res<UiImageResources>,
) {
    // static ONCE: Once = Once::new();
    // ONCE.call_once(|| {
        debug!("Enter main_title_load_images. which image to load?");
        for (id, mut image, mut visibility) in widgets_query.iter_mut() {
            if id == &MAIN_TITLE_BG_GUID || id == &EXTRA_TITLE_BG_GUID {
                visibility.is_visible = true;
            }
            *image = res.0.get(id).unwrap().clone().into();
            debug!(
                "load id: {:?}, image: {:?}, visibility: {:?}",
                id, image, visibility
            );
        }
    // })
}
