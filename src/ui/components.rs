//! All components defined in this file SHOULD named as `Ui${ComponentName}`.
use bevy::prelude::*;

/// `UiWidgetId` is a unique identifier for a widget.
#[component(Default, Deref)]
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
