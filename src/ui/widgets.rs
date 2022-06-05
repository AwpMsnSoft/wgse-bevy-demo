use bevy::{
    prelude::{Button, GlobalTransform, Transform, Visibility},
    ui::{Interaction, Node, Style, UiColor, UiImage},
};

use crate::ui::{components::*, policies::*};

#[policy(Hovering)]
#[generic(Button)]
pub struct BasicButton {
    /// The images be displayed when the button is in different states.
    pub ui_images: UiImageSet,
}
