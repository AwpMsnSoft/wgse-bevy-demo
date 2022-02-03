use bevy::prelude::*;
use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct ExtraMenuRes {
    pub bg_handle: Handle<Image>,
    pub cg_handle: Handle<Image>,
    pub scene_handle: Handle<Image>,
    pub music_handle: Handle<Image>,
    pub exit_handle: Handle<Image>,
}

impl ExtraMenuRes {
    pub(crate) fn new(asset_server: &AssetServer) -> Self {
        let null_handle = asset_server.load("");
        Self {
            bg_handle: asset_server.load("texture/title_ex_2x.png"),
            cg_handle: null_handle.clone(),
            scene_handle: null_handle.clone(),
            music_handle: null_handle.clone(),
            exit_handle: null_handle.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum ExtraMenuSubState {
    Main,
    Cg,
    Event,
    Music,
    Exit,
}

pub(crate) fn extra_menu_spwan(
    mut commands: Commands,
    extra_menu_res: ResMut<ExtraMenuRes>,
    windows: Res<Windows>,
) {
    let windows = windows.get_primary().unwrap();
    commands
        .spawn_bundle(NodeBundle {
            ..Default::default()
        })
        .insert(ForStat {
            states: vec![MainMenuSubState::Extra],
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    image: extra_menu_res.bg_handle.clone().into(),
                    style: Style {
                        size: Size {
                            width: Val::Px(windows.requested_width()),
                            height: Val::Px(windows.requested_height()),
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ForStat {
                    states: vec![MainMenuSubState::Extra],
                });
        });
}

