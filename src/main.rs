// #![windows_subsystem = "windows"]
use bevy::input::mouse::MouseMotion;
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::*;

#[allow(unused_imports)]
#[macro_use]
extern crate generic_widget;

pub(crate) mod cg;
pub(crate) mod media;
pub(crate) mod script;
pub(crate) mod system;
pub(crate) mod text;
pub(crate) mod ui;

fn main() {
    // App::new()
    //     .insert_resource({
    //         WindowDescriptor {
    //             title: String::from("dokoiku HD"),
    //             width: WINDOW_WIDTH,
    //             height: WINDOW_HEIGHT,
    //             scale_factor_override: Some(1.0),
    //             resizable: false,
    //             mode: window::WindowMode::Windowed,
    //             ..Default::default()
    //         }
    //     })
    //     .insert_resource(ClearColor(Color::rgb_u8(255, 255, 255)))
    //     .insert_resource(WgpuSettings {
    //         backends: Some(Backends::PRIMARY),
    //         ..Default::default()
    //     })
    //     .insert_resource(LogSettings {
    //         level: Level::DEBUG,
    //         filter: String::from("wgpu=error,bevy_render=info,naga=info"),
    //     })
    //     .add_startup_system(|windows: Res<Windows>| {
    //         info!("Game initializing...");
    //         debug!("Create window: {:?}", windows.get_primary().unwrap());
    //     })
    //     .add_plugins(DefaultPlugins)
    //     // .add_plugin(ui::main::UIPlugin)
    //     .add_startup_system(setup)
    //     .run();
    App::new()
        .insert_resource({
            WindowDescriptor {
                title: String::from("dokoiku HD"),
                width: 1920.0,
                height: 1080.0,
                scale_factor_override: Some(1.0),
                resizable: false,
                mode: window::WindowMode::Windowed,
                ..Default::default()
            }
        })
        .insert_resource(ClearColor(Color::rgb_u8(255, 255, 255)))
        .insert_resource(WgpuSettings {
            backends: Some(Backends::PRIMARY),
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
        .add_system(test_mouse)
        .run();
}

fn test_mouse(mut mouse: EventReader<CursorMoved>, windows: Res<Windows>) {
    for event in mouse.iter() {
        info!("{:?}", &(event.position, windows.get_primary().unwrap().position()));
    }
}
