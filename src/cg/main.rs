use crate::cg::image::image_display_system;
use bevy::prelude::*;

#[derive(Debug)]
pub struct CgPlugin;

impl Plugin for CgPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(image_display_system);
    }

    fn name(&self) -> &str {
        "wgse_cg"
    }
}
