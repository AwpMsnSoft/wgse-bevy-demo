use bevy::prelude::*;

#[derive(Debug)]
pub struct CgPlugin;

impl Plugin for CgPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(todo!());
    }

    fn name(&self) -> &str {
        "wgse_cg"
    }
}
