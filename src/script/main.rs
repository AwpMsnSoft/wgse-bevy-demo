use crate::script::{
    commands,
    resources::{WgsScript, WgsScriptLoader},
    runtime::{WgsContext, WgsVirtualMachine},
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct ScriptPlugin;

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<WgsScript>()
            .init_asset_loader::<WgsScriptLoader>()
            .add_startup_system(init_wgs_virtualmachine);
    }

    fn name(&self) -> &str {
        "wgse_script"
    }
}

pub fn init_wgs_virtualmachine(
    mut command: Commands,
    scripts: Res<Assets<WgsScript>>,
    contexts: ResMut<Assets<WgsContext>>,
) {
    command
        .spawn()
        .insert(WgsVirtualMachine::init(scripts, contexts));
}
