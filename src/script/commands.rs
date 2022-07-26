//! Handle all wgs script command as rpc call.
//!
//! In order to avoid name collision, Never import this module by `use crate::script::commands::*;`.
//! Consider to import `use crate::script::commands as wgs;` and use commands via `wgs::<CommandName>` instead.
use crate::script::{
    resources::WgsScript,
    runtime::{WgsContext, WgsVirtualMachine},
};
use bevy::prelude::*;

/// Show a dialog on given message box.
///
/// ### format
/// `.message @{chara} "{message}"`
pub struct Message {
    pub chara: String,
    pub message: String,
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
    vm_query: Query<&WgsVirtualMachine>,
    mut trigger: EventReader<Next>,
    mut contexts: ResMut<Assets<WgsContext>>,
) {
    let virtual_machine = vm_query.single();
    let context = contexts
        .get_mut(virtual_machine.context.clone_weak())
        .unwrap();
    for _ in trigger.iter() {
        context.registers.ip += 1;
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
    vm_query: Query<&WgsVirtualMachine>,
    mut next: EventReader<Chain>,
    scripts: Res<Assets<WgsScript>>,
    mut contexts: ResMut<Assets<WgsContext>>,
) {
    let virtual_machine = vm_query.single();
    let context = contexts.get_mut(virtual_machine.context.clone_weak()).unwrap();
    if let Some(path) = next.iter().last() {
        context.load(&path.next, scripts);
    }
}
