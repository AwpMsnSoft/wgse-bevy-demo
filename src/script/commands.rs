//! Handle all wgs script command as rpc call.
//!
//! In order to avoid name collision, Never import this module by `use crate::script::commands::*;`.
//! Consider to import `use crate::script::commands as wgs;` and use commands via `wgs::<CommandName>` instead.

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

/// Change to another script.
/// 
/// ### format
/// 
/// `.chain {script}`
pub struct Chain {
    pub next: String,
}
