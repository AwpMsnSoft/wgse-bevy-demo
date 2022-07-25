use crate::script::runtime::WgsVirtualMachine;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single command which formatted as `.<command> <arg1> <arg2> ...`
#[derive(Debug, Default, Clone)]
pub struct WgsCommand {
    pub command: String,
    pub args: Vec<String>,
}

/// Collection of all `.lable <label>` in the script.
/// Store `(<lable>, <line>)` pairs.
#[derive(Debug, Deref, DerefMut, Default, Clone)]
pub struct WgsLabelMap(pub HashMap<String, usize>);

/// A parsed `.wgs` script file which can be executed directly by `WgsVirtualMachine`.
/// It contains a collection of `WgsCommand` and `WgsLabelMap`.
#[derive(Debug, Default, Clone)]
pub struct WgsParsedScript {
    pub commands: Vec<WgsCommand>,
    pub label_map: WgsLabelMap,
}

/// A `.wgs` script file which be managed by bevy`AssetServer`.
/// - All `.wgs` script files will be loaded and stored as `Bytecode(Vec<u8>)` when the game process initialized.
/// - The `.wgs` script file will be parsed when this asset is first loaded and executed by `WgsVirtualMachine`.
/// After that, the handle of this file will refer to the parsed `WgsParsedScript`.
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "C5BECE68-F078-CBD6-C541-FAC2174C1070"]
pub enum WgsScript {
    Bytecode(Vec<u8>),
    ParsedScript(WgsParsedScript),
}