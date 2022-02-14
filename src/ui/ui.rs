use super::{
    resources::{ExtraTitleRes, MainTitleRes},
    states::UIState,
};
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

pub fn main_title_spawn(
    mut commands: Commands,
    windows: Res<Windows>,
    main_title_res: Res<MainTitleRes>,
) {
    debug!("Enter main_title_spawn.");
    let windows = windows.get_primary().unwrap();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(windows.width() as f32),
                    height: Val::Px(windows.height() as f32),
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UIForStat(vec![MainTitleState::Main.into()]))
        .with_children(|p| {
            // main title background
            p.spawn_bundle(WidgetBundle {
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
                    image: main_title_res.bg_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Main.into()]));
            // 1. start button
            p.spawn_bundle(WidgetBundle {
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
                    image: main_title_res.start_button_hover_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Main.into()]));
            // 2. config button
            p.spawn_bundle(WidgetBundle {
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
                    image: main_title_res.config_button_hover_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Main.into()]));
            // 3. extra button
            p.spawn_bundle(WidgetBundle {
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
                    image: main_title_res.extra_button_hover_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Main.into()]));
            // 4. exit button
            p.spawn_bundle(WidgetBundle {
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
                    image: main_title_res.exit_button_hover_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Main.into()]));
        });
}

pub fn main_title_despawn(
    mut commands: Commands,
    state: Res<State<UIState>>,
    query: Query<(Entity, &UIForStat)>,
) {
    debug!("Enter main_title_despawn.");
    for (entity, states) in &mut query.iter() {
        let state = *state.current();
        debug!("current state: {:?}", state);
        if !states.0.contains(&state.into()) {
            debug!("main_title_despawn: entity: {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}

pub fn extra_title_spawn(
    mut commands: Commands,
    windows: Res<Windows>,
    extra_title_res: Res<ExtraTitleRes>,
) {
    debug!("Enter extra_title_spawn.");
    let windows = windows.get_primary().unwrap();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(windows.width() as f32),
                    height: Val::Px(windows.height() as f32),
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UIForStat(vec![MainTitleState::Extra.into()]))
        .with_children(|p| {
            // extra title background
            p.spawn_bundle(WidgetBundle {
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
                    image: extra_title_res.bg_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Extra.into()]));
            // 1. cg button
            p.spawn_bundle(WidgetBundle {
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
                    image: extra_title_res.cg_button_hover_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Extra.into()]));
            // 2. scene button
            p.spawn_bundle(WidgetBundle {
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
                    image: extra_title_res.scene_button_hover_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Extra.into()]));
            // 3. music button
            p.spawn_bundle(WidgetBundle {
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
                    image: extra_title_res.music_button_hover_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Extra.into()]));
            // 4. back button
            p.spawn_bundle(WidgetBundle {
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
                    image: extra_title_res.back_button_hover_image.clone().into(),
                    ..Default::default()
                },
            })
            .insert(UIForStat(vec![MainTitleState::Extra.into()]));
        });
}

pub fn extra_title_despawn(
    mut commands: Commands,
    state: Res<State<UIState>>,
    query: Query<(Entity, &UIForStat)>,
) {
    debug!("Enter extra_title_despawn.");
    for (entity, states) in &mut query.iter() {
        let state = *state.current();
        debug!("current state: {:?}", state);
        if !states.0.contains(&state.into()) {
            debug!("extra_title_despawn: entity: {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}
