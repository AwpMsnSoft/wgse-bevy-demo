use crate::script::{
    commands as wgs,
    resources::{WgsScript, WgsScriptLoader},
    runtime::{wgse_event_dispatcher, WgsContext, WgsEvent, WgsVirtualMachine},
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct ScriptPlugin;

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<WgsScript>()
            .init_asset_loader::<WgsScriptLoader>()
            .add_event::<wgs::Message>()
            .add_event::<wgs::Chain>()
            .add_event::<wgs::Lable>()
            .add_event::<wgs::Next>()
            .add_event::<wgs::Exit>()
            .add_event::<WgsEvent>()
            .add_startup_system(init_wgs_virtualmachine)
            .add_system(wgse_event_dispatcher)
            .add_system(wgs::wgse_command_system_next)
            .add_system(wgs::wgse_command_system_chain)
            .add_system(wgs::wgse_command_system_exit);
    }

    fn name(&self) -> &str {
        "wgse_script"
    }
}

pub fn init_wgs_virtualmachine(mut command: Commands, scripts: Res<Assets<WgsScript>>) {
    debug!("Initializing wgs script virtual machine.");
    command
        .spawn()
        .insert(WgsVirtualMachine::init())
        .insert(WgsContext::init(&scripts));
}
