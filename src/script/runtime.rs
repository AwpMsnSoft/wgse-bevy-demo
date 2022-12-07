use crate::script::commands as wgs;
use crate::script::resources::{WgsCommand, WgsScript};
use bevy::prelude::*;
use sscanf::scanf;

/// Registers of current wgs script context.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct WgsRegisters {
    /// Instructions Register, which is the current excecuting line.
    pub ip: usize,
    /// Next panel used by wgs command.
    pub pn: i32,
    // Add more registers here.
}

impl WgsRegisters {
    pub fn reset(&mut self) {
        self.ip = 0;
        self.pn = 0;
    }
}

/// A virtual machine context which bind to current executing `.wgs` script.
/// This struct is also used to save game status.
///
/// There is only one `WgsContext` in a game process actually.
#[derive(Debug, Clone, Component, Hash, PartialEq, Eq)]
pub struct WgsContext {
    pub script: Handle<WgsScript>,
    pub registers: WgsRegisters,
}

impl WgsContext {
    pub fn init(asset_server: Res<AssetServer>) -> Self {
        Self {
            // TODO: set init script's path by `WgsScriptSettings` resource.
            script: asset_server.load("script/init.wgs"),
            registers: WgsRegisters::default(),
        }
    }

    pub fn load(&mut self, path: &str, asset_server: Res<AssetServer>) {
        debug!("trying load script/{}.wgs", path);
        self.script = asset_server.load(format!("script/{}.wgs", path));
        self.registers.reset();
    }
}

/// A virtual machine which can execute `.wgs` script.
///
/// `WgsVirtualMachine` stores a handle of current executing `WgsContext` and provide a set of commands to execute
/// `.wgs` script. By default, a virtual machine will load `init.wgs` script when the game initialized,
///  and a game process only have one virtual machine.
///
/// WgsVirtualMachine is managed as a bevy `Component`.
#[derive(Debug, Clone, Component)]
pub struct WgsVirtualMachine {}

impl WgsVirtualMachine {
    pub fn init() -> Self {
        Self {}
    }

    pub fn exec(&self, cmd: WgsCommand, ev: &mut EventWriter<WgsEvent>) {
        match cmd.command.as_str() {
            "message" => ev.send(WgsEvent::Message(cmd)),
            "chain" => ev.send(WgsEvent::Chain(cmd)),
            "label" => ev.send(WgsEvent::Lable(cmd)),
            "next" => ev.send(WgsEvent::Next(cmd)),
            "exit" => ev.send(WgsEvent::Exit(cmd)),
            "cg" => ev.send(WgsEvent::Cg(cmd)),
            "panel" => ev.send(WgsEvent::Panel(cmd)),
            _ => panic!("Unknown command: {}", cmd.command),
        }
    }
}

/// Dispatch
///
#[derive(Debug)]
pub enum WgsEvent {
    // no args event
    Lable(WgsCommand),
    Exit(WgsCommand),
    // one arg event
    Next(WgsCommand),
    Chain(WgsCommand),
    Cg(WgsCommand),
    Panel(WgsCommand),
    // multi args event
    Message(WgsCommand),
}

// Add all commands here.
// TODO: use proc_macro to generate command handlers.
pub fn wgse_event_dispatcher(
    mut ev: EventReader<WgsEvent>,
    mut message: EventWriter<wgs::Message>,
    mut chain: EventWriter<wgs::Chain>,
    mut lable: EventWriter<wgs::Lable>,
    // mut next: EventWriter<wgs::Next>,
    mut cg: EventWriter<wgs::Cg>,
    mut panel: EventWriter<wgs::Panel>,
    mut exit: EventWriter<wgs::Exit>,
) {
    for event in ev.iter() {
        debug!("Dispatching event: {:?}", event);
        match event {
            WgsEvent::Lable(cmd) => lable.send(wgs::Lable {
                lable: cmd.args[0].clone(),
            }),
            WgsEvent::Next(_) => (), /* actually do nothing */
            // WgsEvent::Next(_) => next.send(wgs::Next {}),
            WgsEvent::Exit(_) => exit.send(wgs::Exit {}),
            WgsEvent::Chain(cmd) => chain.send(wgs::Chain {
                next: cmd.args[0].clone(),
            }),
            WgsEvent::Message(cmd) => message.send(wgs::Message {
                chara: scanf!(cmd.args[0], "@{}", String).unwrap(),
                message: cmd
                    .args
                    .iter()
                    .skip(1)
                    .fold(String::new(), |acc, x| acc + x + " "),
            }),
            WgsEvent::Cg(cmd) => cg.send(wgs::Cg {
                path: cmd.args[0].clone(),
            }),
            WgsEvent::Panel(cmd) => panel.send(wgs::Panel(cmd.args[0].parse::<i32>().unwrap())),
        }
    }
}
