use crate::script::resources::WgsScript;
use bevy::{prelude::*, reflect::TypeUuid};

/// Registers of current wgs script context.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct WgsRegisters {
    /// Instructions Register, which is the current excecuting line.
    pub ip: usize,
    // Add more registers here.
}

/// A virtual machine context which bind to current executing `.wgs` script.
/// This struct is also used to save game status.
///
/// WgsContext is managed by bevy `AssetServer`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, TypeUuid)]
#[uuid = "B12D9C66-94BE-6715-9F29-1491D91D4CC5"]
pub struct WgsContext {
    pub script: Handle<WgsScript>,
    pub registers: WgsRegisters,
}

/// A virtual machine which can execute `.wgs` script.
///
/// `WgsVirtualMachine` stores a handle of current executing `WgsContext` and provide a set of commands to execute `.wgs` script.
/// By default, a virtual machine will load `init.wgs` script when the game initialized, and a game process only have one virtual machine.
#[derive(Debug, Clone)]
pub struct WgsVirtualMachine {
    /// Current executing context.
    pub context: Handle<WgsContext>,
}

impl WgsVirtualMachine {
    pub fn new() -> Self {
        todo!()
    }

    pub fn load(&self, asset_server: &AssetServer) {
        // TODO
        let script: Handle<WgsScript> = asset_server.load("assets/script/init.wgs");
    }

    pub fn next(&self) {
        println!("next");
    }
}
