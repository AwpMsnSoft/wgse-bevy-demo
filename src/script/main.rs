use crate::script::{commands, runtime::WgsVirtualMachine};
use bevy::prelude::*;

pub fn load_system(asset_server: Res<AssetServer>, mut wgs_vm: ResMut<WgsVirtualMachine>) {
    wgs_vm.load(&asset_server);
}

pub fn next_system(mut next: EventReader<commands::Next>, mut wgs_vm: ResMut<WgsVirtualMachine>) {
    for _ in next.iter() {
        wgs_vm.next();
    }
}
