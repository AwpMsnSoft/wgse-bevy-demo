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
            .add_event::<wgs::Image>()
            .add_event::<wgs::Panel>()
            .add_event::<WgsEvent>()
            .add_startup_system(init_wgs_virtualmachine)
            .add_system(wgse_event_dispatcher)
            .add_system(wgs::wgse_command_system_next)
            .add_system(wgs::wgse_command_system_chain)
            .add_system(wgs::wgse_command_system_exit)
            .add_system(wgs::wgse_command_system_panel);
    }

    fn name(&self) -> &str {
        "wgse_script"
    }
}

pub fn init_wgs_virtualmachine(mut command: Commands, asset_server: Res<AssetServer>) {
    debug!("Initializing wgs script virtual machine.");
    command.spawn((WgsVirtualMachine::init(), WgsContext::init(asset_server)));
}
