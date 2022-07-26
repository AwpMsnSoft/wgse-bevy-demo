use crate::script::commands as wgs;
use crate::script::resources::WgsScript;
use bevy::{prelude::*, reflect::TypeUuid};

/// Registers of current wgs script context.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct WgsRegisters {
    /// Instructions Register, which is the current excecuting line.
    pub ip: usize,
    // Add more registers here.
}

impl WgsRegisters {
    pub fn reset(&mut self) {
        self.ip = 0;
    }
}

/// A virtual machine context which bind to current executing `.wgs` script.
/// This struct is also used to save game status.
///
/// WgsContext is managed by bevy `AssetServer`.
///
/// There is only one `WgsContext` in a game process actually.
#[derive(Debug, Clone, Hash, PartialEq, Eq, TypeUuid)]
#[uuid = "B12D9C66-94BE-6715-9F29-1491D91D4CC5"]
pub struct WgsContext {
    pub script: Handle<WgsScript>,
    pub registers: WgsRegisters,
}

impl WgsContext {
    pub fn init(scripts: Res<Assets<WgsScript>>) -> Self {
        Self {
            // TODO: set init script's path by `WgsScriptSettings` resource.
            script: scripts.get_handle("script/init.wgs").clone_weak(),
            registers: WgsRegisters::default(),
        }
    }

    pub fn load(&mut self, path: &str, scripts: Res<Assets<WgsScript>>) {
        self.script = scripts.get_handle(path).clone_weak();
        self.registers.reset();
    }
}

/// A virtual machine which can execute `.wgs` script.
///
/// `WgsVirtualMachine` stores a handle of current executing `WgsContext` and provide a set of commands to execute `.wgs` script.
/// By default, a virtual machine will load `init.wgs` script when the game initialized, and a game process only have one virtual machine.
///
/// WgsVirtualMachine is managed as a bevy `Component`.
#[derive(Debug, Clone, Component)]
pub struct WgsVirtualMachine {
    /// Current executing context.
    pub context: Handle<WgsContext>,
}

impl WgsVirtualMachine {
    pub fn init(scripts: Res<Assets<WgsScript>>, mut contexts: ResMut<Assets<WgsContext>>) -> Self {
        Self {
            context: contexts.add(WgsContext::init(scripts)),
        }
    }
}

pub fn exec(
    vm_query: Query<&WgsVirtualMachine>,
    mut contexts: ResMut<Assets<WgsContext>>,
    scripts: Res<Assets<WgsScript>>,
) {
    let virtual_machine = vm_query.single();
    let context = contexts
        .get_mut(virtual_machine.context.clone_weak())
        .unwrap();
    let script = scripts.get(context.script.clone_weak()).unwrap();
}
