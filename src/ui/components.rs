//! All components defined in this file SHOULD named as `Ui${ComponentName}`.
use bevy::{prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};

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

/// `UiWidgetId` is a unique identifier for a widget.
#[component(Default, Deref, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UiWidgetId(pub i32);

#[component(Deref, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiEntity(pub Option<Entity>);

#[component(Deref, PartialEq, Eq)]
pub struct UiEntitySet(pub HashSet<Entity>);

#[component(Default, Deref, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UiBoolean(pub bool);

#[component(Default, Deref, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UiFloat(pub f32);
