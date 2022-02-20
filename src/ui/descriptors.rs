use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component, PartialEq, Eq, Hash)]
pub struct WidgetID(pub i32);

impl Default for WidgetID {
    fn default() -> Self {
        WidgetID(0)
    }
}

#[derive(Bundle, Debug, Serialize, Deserialize)]
pub struct WidgetBundle<SubBundle>
where
    SubBundle: Send + Sync + 'static + Bundle,
{
    pub id: WidgetID,
    #[bundle]
    pub children: SubBundle,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Descriptor {
    pub id: i32,
    #[serde(flatten)]
    pub content: WidgetDescriptor,
}

#[derive(Debug, Serialize, Deserialize, Component)]
#[allow(non_camel_case_types)]
pub enum WidgetDescriptor {
    button(ButtonDescriptor),
    // group(Vec<Descriptor>),
}

#[derive(Debug, Serialize, Deserialize, Component)]
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
        ButtonBundle::from(ButtonDescriptor {
            size: Vec2::new($size_x, $size_y),
            position: Vec2::new($position_x, $position_y),
        })
    }};
}
