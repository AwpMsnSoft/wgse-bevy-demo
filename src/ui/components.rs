//! All components defined in this file SHOULD named as `Ui${ComponentName}`.
use crate::ui::widgets::BasicButton;
use bevy::prelude::*;

/// `UiWidgetId` is a unique identifier for a widget.
#[component(Default, Deref, PartialEq, Eq, Hash)]
pub struct UiWidgetId(pub i32);

/// `UiImageSet` is a component that contains three optional handles to `Image` resources.
///
/// Properties:
///
/// * `image0`: The image that is displayed when the button is in the default state.
/// * `image1`: The image that is displayed when the button is hovered.
/// * `image2`: The image that is displayed when the button is clicked.
#[component(Default)]
pub struct UiImageSet {
    pub image0: Option<Handle<Image>>,
    pub image1: Option<Handle<Image>>,
    pub image2: Option<Handle<Image>>,
}

#[component(Default, Deref, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiInteger(pub i32);

#[component(Default, Deref, PartialEq, PartialOrd)]
pub struct UiFloat(pub f32);

#[component(Default, Deref, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiBool(pub bool);