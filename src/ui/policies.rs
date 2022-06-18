use crate::ui::components::*;
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
}

#[policy]
pub struct Selectable {
    pub _0: UiInteger,
}