#[windows_subsystem = "windows"]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_system(|| println!("Hello, world!"))
        .run()
}