use crate::ui::{components::*, policies::*};
use bevy::{
    prelude::{Bundle, Button, GlobalTransform, Transform, Visibility},
    ui::{Interaction, Node, Style, UiColor, UiImage},
};

#[policy(Hoverable)]
#[generic(Button)]
#[derive(Debug, Clone)]
pub struct BasicButton {
    /// The images be displayed when the button is in different states.
    pub ui_images: UiImageSet,
}

#[policy(Slidable)]
#[generic(Button)]
pub struct Slider {
    /// Cursor position of the slide, [0, 1].
    pub cursor: UiFloat,
}

#[policy(Switchable)]
#[derive(Bundle)]
pub struct Switch {
    /// Current active child entity.
    pub active_child: UiEntity,
}

#[policy(SubSwitchable)]
#[generic(Button)]
/// Add this bundle to a `Switch` by `insert_children`.
pub struct SubSwitch {}