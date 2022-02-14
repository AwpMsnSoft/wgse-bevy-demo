use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component, PartialEq, Eq)]
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

impl Into<ButtonBundle> for ButtonDescriptor {
    fn into(self) -> ButtonBundle {
        ButtonBundle {
            node: Node { size: self.size },
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    left: Val::Px(self.position.x),
                    right: Val::Px(self.position.x + self.size.x),
                    top: Val::Px(self.position.y),
                    bottom: Val::Px(self.position.y + self.size.y),
                },
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        }
    }
}

#[test]
fn test_serialize_button_descriptor() {
    let descriptor = Descriptor {
        id: 1,
        content: WidgetDescriptor::button(ButtonDescriptor {
            size: Vec2::new(100.0, 100.0),
            position: Vec2::new(10.0, 10.0),
        }),
    };

    let serialized = serde_json::to_string(&descriptor).unwrap();
    println!("{}", serialized);
}

#[test]
fn test_deserialize_button_descriptor() {
    let serialized = r#"
    {
        "id": 1,
        "button": {
            "size": [100, 100],
            "position": [10, 10]
        }
    }
    "#;

    let descriptor: Descriptor = serde_json::from_str(serialized).unwrap();
    println!("{:?}", descriptor);
}