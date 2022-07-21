use crate::script::handles::WgsScript;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WgsVirtualMachine {}

impl WgsVirtualMachine {
    pub fn new() -> Self {
        WgsVirtualMachine {}
    }

    pub fn load(&self, asset_server: &AssetServer) {
        // TODO
        let script: Handle<WgsScript> = asset_server.load("assets/script/init.wgs");
    }

    pub fn next(&self) {
        println!("next");
    }
}
