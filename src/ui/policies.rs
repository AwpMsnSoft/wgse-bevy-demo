use crate::ui::{components::*, widgets::*};
use bevy::prelude::*;
use bevy::ui::Interaction;

#[policy]
pub struct Hoverable {
    pub _0: UiImageSet,
    pub _1: Interaction,
}

#[policy]
pub struct Slidable {
    pub _0: UiImage,
    pub _1: UiFloat,
}

#[policy]
pub struct Switchable {
    pub _0: UiEntity,
}

#[policy]
pub struct SubSwitchable {}
