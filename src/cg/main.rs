use crate::cg::background::background_display_system;
use bevy::prelude::*;

#[derive(Debug)]
pub struct CgPlugin;

impl Plugin for CgPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(background_display_system);
    }

    fn name(&self) -> &str {
        "wgse_cg"
    }
}
