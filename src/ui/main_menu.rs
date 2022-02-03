use bevy::prelude::*;

/// main window layout

#[derive(Debug)]
pub(crate) struct MainMenuRes {
    pub bg_handle: Handle<Image>,
    pub start_handle: Handle<Image>,
    pub config_handle: Handle<Image>,
    pub extra_handle: Handle<Image>,
    pub exit_handle: Handle<Image>,
}

impl MainMenuRes {
    pub(crate) fn new(asset_server: &AssetServer) -> Self {
        let null_handle = asset_server.load("");
        Self {
            bg_handle: asset_server.load("textures/title_sel0_2x.png"),
            start_handle: null_handle.clone(),
            config_handle: null_handle.clone(),
            extra_handle: null_handle.clone(),
            exit_handle: null_handle.clone(),
        }
    }
}

#[derive(Component)]
pub(crate) struct ForStat<T> {
    pub(crate) states: Vec<T>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum MainMenuSubState {
    Main,
    Start,
    Config,
    Extra,
    Exit,
}

pub(crate) fn main_menu_spawn(mut commands: Commands, main_menu_res: ResMut<MainMenuRes>, windows: Res<Windows>) {
    let windows = windows.get_primary().unwrap();
    commands
        .spawn_bundle(NodeBundle {
            ..Default::default()
        })
        .insert(ForStat {
            states: vec![MainMenuSubState::Main],
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: main_menu_res.bg_handle.clone().into(),
                style: Style {
                    size: Size {
                        width: Val::Px(windows.requested_width()),
                        height: Val::Px(windows.requested_height())
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
