//! Handle all wgs script command as rpc call. 
use bevy::prelude::*;

pub struct Message {
    pub chara: String,
    pub message: String,
}

pub enum WgsInnerCommand {
    Next
}