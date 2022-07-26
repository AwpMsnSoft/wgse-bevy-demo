// #![windows_subsystem = "windows"]
use crate::ui::{
    resources::{FontResources, UiImageResources},
    ui::{WINDOW_HEIGHT, WINDOW_WIDTH},
};
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;

pub(crate) mod cg;
pub(crate) mod media;
pub(crate) mod script;
pub(crate) mod system;
pub(crate) mod text;
pub(crate) mod ui;

#[bevy_main]
fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
    }
    App::new()
        .insert_resource({
            WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..Default::default()
            }
        })
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .insert_resource(LogSettings {
            level: Level::DEBUG,
            filter: String::from("wgpu=error,bevy_render=info,naga=info"),
        })
        .add_startup_system(|windows: Res<Windows>| {
            info!("Game initializing...");
            debug!("Create window: {:?}", windows.get_primary().unwrap());
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::main::UiPlugin)
        .add_plugin(script::main::ScriptPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn_bundle(UiCameraBundle::default());
    command.insert_resource(UiImageResources::new(&asset_server));
    command.insert_resource(FontResources::new(&asset_server));
}
