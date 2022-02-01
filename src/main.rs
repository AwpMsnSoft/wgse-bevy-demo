// #![windows_subsystem = "windows"]

use bevy::{prelude::*, window::*};

pub(crate) mod cg;
pub(crate) mod media;
pub(crate) mod script;
pub(crate) mod text;
pub(crate) mod ui;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("何処へ行くの、あの日"),
            width: 800.0,
            height: 600.0,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb_u8(255, 255, 255)))
        .add_plugins(DefaultPlugins)
        .add_state(ui::main_menu::MainMenuSubState::None)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_enter(ui::main_menu::MainMenuSubState::None)
                .with_system(ui::main_menu::main_menu_spawn),
        )
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ui::main_menu::MainMenuRes::new(&asset_server));
}
