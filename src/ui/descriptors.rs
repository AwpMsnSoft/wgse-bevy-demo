use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::resources::UiImageResources;

#[derive(Debug, Serialize, Deserialize, Component, PartialEq, Eq, Hash)]
pub struct WidgetId(pub i32);

impl Default for WidgetId {
    fn default() -> Self {
        WidgetId(0)
    }
}

#[derive(Bundle, Debug, Serialize, Deserialize)]
pub struct WidgetBundle<SubBundle>
where
    SubBundle: Send + Sync + 'static + Bundle,
{
    pub id: WidgetId,
    #[bundle]
    pub children: SubBundle,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Descriptor {
    pub id: i32,
    #[serde(flatten)]
    pub content: WidgetDescriptor,
}

#[macro_export]
macro_rules! descriptor {
    ($id: expr, $content: expr) => {{
        Descriptor {
            id: $id,
            content: $content,
        }
    }};
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
#[allow(non_camel_case_types)]
pub enum WidgetDescriptor {
    button(ButtonDescriptor),
    group(Vec<Descriptor>),
    image(ImageDescriptor),
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct ButtonDescriptor {
    pub size: Vec2,
    pub position: Vec2,
}

impl From<ButtonDescriptor> for ButtonBundle {
    fn from(descriptor: ButtonDescriptor) -> Self {
        ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Px(descriptor.size.x),
                    height: Val::Px(descriptor.size.y),
                },
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(descriptor.position.x),
                    right: Val::Px(descriptor.position.x + descriptor.size.x),
                    top: Val::Px(descriptor.position.y),
                    bottom: Val::Px(descriptor.position.y - descriptor.size.y),
                },
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        }
    }
}

#[macro_export]
macro_rules! button {
    ($size_x: expr, $size_y: expr, $position_x: expr, $position_y: expr) => {{
        ButtonDescriptor {
            size: Vec2::new($size_x, $size_y),
            position: Vec2::new($position_x, $position_y),
        }
    }};
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Scale2D(pub Vec2);

impl Default for Scale2D {
    fn default() -> Self {
        Scale2D(Vec2::new(1.0, 1.0))
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, Component)]
pub struct ImageDescriptor {
    pub size: Vec2,
    pub position: Vec2,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default)]
    pub scale: Scale2D,
    pub image: String,
}

impl ImageDescriptor {
    pub fn load_resources(self, resources: &Res<UiImageResources>) -> ImageBundle {
        ImageBundle {
            style: Style {
                size: Size {
                    width: Val::Px(self.size.x),
                    height: Val::Px(self.size.y),
                },
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(self.position.x),
                    right: Val::Px(self.position.x + self.size.x),
                    top: Val::Px(self.position.y),
                    bottom: Val::Px(self.position.y - self.size.y),
                },
                ..Default::default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_z(self.rotation),
                scale: Vec3::new(self.scale.0.x, self.scale.0.y, 1.0),
                ..Default::default()
            },
            image: resources.0.get(self.image.as_str()).unwrap().clone().into(),
            ..Default::default()
        }
    }
}

#[macro_export]
macro_rules! image {
    ($size_x: expr, $size_y: expr, $position_x: expr, $position_y: expr, $image: expr) => {{
        ImageDescriptor {
            size: Vec2::new($size_x, $size_y),
            position: Vec2::new($position_x, $position_y),
            image: String::from($image),
            ..Default::default()
        }
    }};
}

#[test]
fn test_image_serialization() {
    let image = Descriptor {
        id: 0,
        content: WidgetDescriptor::image(image!(100.0, 100.0, 0.0, 0.0, "test.png")),
    };
    let serialized = serde_json::to_string(&image).unwrap();
    println!("{}", serialized);
}
