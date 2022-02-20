// #![windows_subsystem = "windows"]
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy::render::options::{Backends, WgpuOptions};
use bevy::*;

pub(crate) mod cg;
pub(crate) mod media;
pub(crate) mod script;
pub(crate) mod system;
pub(crate) mod text;
pub(crate) mod ui;

fn main() {
    App::new()
        .insert_resource({
            WindowDescriptor {
                title: String::from("dokoiku HD"),
                width: 800.0,
                height: 600.0,
                scale_factor_override: Some(1.0),
                vsync: true,
                resizable: false,
                mode: window::WindowMode::Windowed,
                ..Default::default()
            }
        })
        .insert_resource(ClearColor(Color::rgb_u8(255, 255, 255)))
        .insert_resource(WgpuOptions {
            backends: Some(Backends::DX12),
            ..Default::default()
        })
        .insert_resource(LogSettings {
            level: Level::DEBUG,
            filter: String::from("wgpu=error,bevy_render=info,naga=info"),
        })
        .add_startup_system(|windows: Res<Windows>| {
            info!("Game initializing...");
            debug!("Create window: {:?}", windows.get_primary().unwrap());
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::main::UIPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn_bundle(UiCameraBundle::default());
    command.insert_resource(ui::resources::UiImageResources::new(&asset_server));
}
