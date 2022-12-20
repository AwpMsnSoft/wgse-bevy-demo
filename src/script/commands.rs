//! Handle all wgs script command as rpc call.
//!
//! In order to avoid name collision, Never import this module by `use crate::script::commands::*;`.
//! Consider to import `use crate::script::commands as wgs;` and use commands via `wgs::<CommandName>` instead.
use crate::{
    script::{
        resources::WgsScript,
        runtime::{WgsContext, WgsEvent, WgsVirtualMachine},
    },
    ui::descriptors::WidgetId,
};
use bevy::{app::AppExit, prelude::*};

/// Show a dialog on given message box.
///
/// ### format
/// `.message @{chara} "{message}"`
pub struct Message {
    pub chara: String,
    pub message: String,
}

/// Show an image on given panel
///
/// ### format
/// `.image {filename}`
pub struct Image {
    pub path: String,
}

/// Create a new label.
///
/// In current `.wgs` engine version, we suggest to use `.label` at the beginning of each logic block.
///
/// ### Example
/// ```bash
/// .lable 1000
/// .message @A "This is a dialog."
/// .message @B "This is the response."
/// .tachie @left "pa_normal_01.png"
/// ```
///
/// ### format
/// `.label {lable}`
pub struct Lable {
    pub lable: String,
}

/// Get next command from script.
///
/// ### format
///
/// `.next`
///
/// ### attention
///
/// This command is a dummy command which only used to control flow of script.
pub struct Next {}

pub fn wgse_command_system_next(
    mut query: Query<(&WgsVirtualMachine, &mut WgsContext)>,
    scripts: Res<Assets<WgsScript>>,
    mut trigger: EventReader<Next>,
    mut ev: EventWriter<WgsEvent>,
) {
    if trigger.iter().last().is_some() {
        let (virtual_machine, mut context) = query.single_mut();
        if let Some(script) = scripts.get(&context.script.clone()) {
            virtual_machine.exec(
                script.0.commands[context.registers.ip as usize].clone(),
                &mut ev,
            );
            context.registers.ip += 1;
        }
    }
}

/// Change to another script.
///
/// ### format
///
/// `.chain {script}`
pub struct Chain {
    pub next: String,
}

pub fn wgse_command_system_chain(
    mut query: Query<(&WgsVirtualMachine, &mut WgsContext)>,
    mut chain: EventReader<Chain>,
    asset_server: Res<AssetServer>,
) {
    let (_, mut context) = query.single_mut();
    if let Some(path) = chain.iter().last() {
        context.load(&path.next, asset_server);
    }
}

/// Select the next panel will be used.
///
/// ### format
/// `.panel $panel_id`
pub struct Panel(pub i32);

pub fn wgse_command_system_panel(
    mut query: Query<(&WgsVirtualMachine, &mut WgsContext)>,
    mut widgets: Query<&WidgetId>,
    mut panel: EventReader<Panel>,
    mut trigger: EventWriter<Next>,
) {
    let (_, mut context) = query.single_mut();
    if let Some(panel) = panel.iter().last() {
        context.registers.pn = panel.0;
        debug!("Registers changed to {:?}", context.registers);
        // Special options for some kinds of panel...
        match panel.0 {
            _ => { /* do nothing */ }
        }
        trigger.send(Next {});
    }
}

/// Exit the game process
///
/// ### format
/// `.exit`
pub struct Exit {}

pub fn wgse_command_system_exit(
    mut _query: Query<(&WgsVirtualMachine, &mut WgsContext)>,
    mut trigger: EventReader<Exit>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    trigger.iter().for_each(|_| app_exit_event.send(AppExit));
}
