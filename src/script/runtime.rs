use crate::script::commands as wgs;
use crate::script::resources::WgsScript;
use bevy::{asset::AssetPath, prelude::*, reflect::TypeUuid};

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
            script: scripts.get_handle("script/init.wgs").clone_weak(),
            registers: WgsRegisters::default(),
        }
    }

    pub fn load(&mut self, script: Handle<WgsScript>) {
        self.script = script; // TODO: check if the script is parsed or not.
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

    pub fn load(
        &mut self,
        next: EventReader<wgs::Chain>,
        scripts: Res<Assets<WgsScript>>,
        mut contexts: ResMut<Assets<WgsContext>>,
    ) {
        let mut context = contexts.get_mut(self.context).unwrap();
        if let Some(path) = next.iter().last() {
            context.load(scripts.get_handle(path.next).clone_weak());
        }
    }

    pub fn next(&mut self, mut trigger: EventReader<wgs::Next>) {
        for _ in trigger.iter() {
            todo!()
        }
    }
}
