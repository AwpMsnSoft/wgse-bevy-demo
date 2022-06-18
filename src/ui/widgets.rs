use crate::ui::{components::*, policies::*};
use bevy::{
    prelude::{Bundle, Button, GlobalTransform, Transform, Visibility, ButtonBundle},
    ui::{Interaction, Node, Style, UiColor, UiImage},
};

#[policy(Hoverable)]
#[generic(Button)]
pub struct BasicButton {
    /// The images be displayed when the button is in different states.
    pub ui_images: UiImageSet,
}

#[generic(Button)]
pub struct BasicSlide {
    /// Cursor position of the slide, [0, 1].
    pub cursor: UiFloat,
}

#[derive(Bundle)]
pub struct BasicButtonSet2 {
    #[bundle]
    pub button0: BasicButton,
    #[bundle]
    pub button1: BasicButton,
}
