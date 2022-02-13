use super::{
    resources::{ExtraTitleRes, MainTitleRes, CONFIG_BUTTON_GUID},
    states::{ExtraTitleState, MainTitleState, UIForStat, UIState},
    widget,
};
use bevy::prelude::*;

pub fn main_title_spawn(
    mut commands: Commands,
    windows: Res<Windows>,
    main_title_res: Res<MainTitleRes>,
) {
    info!("Enter main_title_spawn.");
    let windows = windows.get_primary().unwrap();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(windows.width() as f32),
                    height: Val::Px(windows.height() as f32),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UIForStat(vec![MainTitleState::Main.into()]))
        .with_children(|p| {
            p.spawn_bundle(ImageBundle {
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
            });
            p.spawn_bundle(widget::WidgetBundle::<ButtonBundle> {
                id: widget::WidgetID(CONFIG_BUTTON_GUID),
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
            });
        });
}

pub fn main_title_despawn(
    mut commands: Commands,
    state: Res<State<UIState>>,
    query: Query<(Entity, &UIForStat)>,
) {
    info!("Enter main_title_despawn.");
    for (entity, states) in &mut query.iter() {
        let state = *state.current();
        if !states.0.contains(&state.into()) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn extra_title_spawn(
    mut commands: Commands,
    windows: Res<Windows>,
    extra_title_res: Res<ExtraTitleRes>,
) {
    info!("Enter extra_title_spawn.");
    let windows = windows.get_primary().unwrap();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(windows.width() as f32),
                    height: Val::Px(windows.height() as f32),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UIForStat(vec![MainTitleState::Extra.into()]))
        .with_children(|p| {
            p.spawn_bundle(ImageBundle {
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
            });
        });
}

pub fn extra_title_despawn(
    mut commands: Commands,
    state: Res<State<UIState>>,
    query: Query<(Entity, &UIForStat)>,
) {
    info!("Enter extra_title_despawn.");
    for (entity, states) in &mut query.iter() {
        let state = *state.current();
        if !states.0.contains(&state.into()) {
            commands.entity(entity).despawn();
        }
    }
}
