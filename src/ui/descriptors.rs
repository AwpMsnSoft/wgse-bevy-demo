use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Component, PartialEq, Eq, Hash)]
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
    #[serde(default)]
    pub children: Option<GroupDescriptor>,
}

#[macro_export]
macro_rules! descriptor {
    ($id: expr, $content: expr, $children: expr) => {{
        Descriptor {
            id: $id,
            content: $content,
            children: $children,
        }
    }};
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
#[allow(non_camel_case_types)]
pub enum WidgetDescriptor {
    button(ButtonDescriptor),
    image(ImageDescriptor),
    text(TextDescriptor),
}

#[allow(unreachable_patterns)]
// TODO: use a macro to generate this
pub fn widget_descriptor_spawn(parent: &mut ChildBuilder, descriptor: &Descriptor) {
    // Spawn main widget
    let mut main_command = match &descriptor.content {
        WidgetDescriptor::button(button) => {
            debug!("Spawning button {:?}: {:?}", descriptor.id, button);
            parent.spawn(WidgetBundle {
                id: WidgetId(descriptor.id),
                children: ButtonBundle::from(button.clone()),
            })
        }
        WidgetDescriptor::image(image) => {
            debug!("Spawning image {:?}: {:?}", descriptor.id, image);
            parent.spawn(WidgetBundle {
                id: WidgetId(descriptor.id),
                children: ImageBundle::from(image.clone()),
            })
        }
        WidgetDescriptor::text(text) => {
            debug!("Spawning text {:?}: {:?}", descriptor.id, text);
            parent.spawn(WidgetBundle {
                id: WidgetId(descriptor.id),
                children: TextBundle::from(text.clone()),
            })
        }
        _ => panic!("Current WidgetDescriptor is not supported yet."),
    };
    // Spawn children widgets (if any)
    if let Some(children) = &descriptor.children {
        debug!("Spawning children as follows:");
        main_command.with_children(|parent| {
            for child in children.0.iter() {
                widget_descriptor_spawn(parent, child);
            }
        });
    };
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Component)]
pub struct ButtonDescriptor {
    pub size: Vec2,
    pub position: Vec2,
    pub z_index: i32,
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
                position: UiRect {
                    left: Val::Px(descriptor.position.x),
                    right: Val::Px(descriptor.position.x + descriptor.size.x),
                    top: Val::Px(descriptor.position.y),
                    bottom: Val::Px(descriptor.position.y - descriptor.size.y),
                },
                ..Default::default()
            },
            z_index: ZIndex::Global(descriptor.z_index),
            ..Default::default()
        }
    }
}

#[macro_export]
macro_rules! button {
    ($size: expr, $position: expr, $z_index: expr) => {{
        ButtonDescriptor {
            size: Vec2::from($size),
            position: Vec2::from($position),
            z_index: $z_index,
        }
    }};
}

#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Component)]
pub struct Scale2D(pub Vec2);

impl Default for Scale2D {
    fn default() -> Self {
        Scale2D(Vec2::new(1.0, 1.0))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Component)]
pub struct ImageDescriptor {
    pub size: Vec2,
    pub position: Vec2,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default)]
    pub scale: Scale2D,
    pub z_index: i32,
}

impl From<ImageDescriptor> for ImageBundle {
    fn from(descriptor: ImageDescriptor) -> Self {
        Self {
            style: Style {
                size: Size {
                    width: Val::Px(descriptor.size.x),
                    height: Val::Px(descriptor.size.y),
                },
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(descriptor.position.x),
                    right: Val::Px(descriptor.position.x + descriptor.size.x),
                    top: Val::Px(descriptor.position.y),
                    bottom: Val::Px(descriptor.position.y - descriptor.size.y),
                },
                ..Default::default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_z(descriptor.rotation),
                scale: Vec3::new(descriptor.scale.0.x, descriptor.scale.0.y, 1.0),
                ..Default::default()
            },
            z_index: ZIndex::Global(descriptor.z_index),
            ..Default::default()
        }
    }
}

#[macro_export]
macro_rules! image {
    ($size: expr, $position: expr, $z_index: expr) => {{
        ImageDescriptor {
            size: Vec2::from($size),
            position: Vec2::from($position),
            z_index: $z_index,
            ..Default::default()
        }
    }};
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Component)]
pub struct FontSettings {
    pub size: f32,
    pub color: Vec4,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Component)]
pub struct TextDescriptor {
    pub size: Vec2,
    pub position: Vec2,
    #[serde(default)]
    pub font: FontSettings,
    pub z_index: i32,
}

impl From<TextDescriptor> for TextBundle {
    fn from(descriptor: TextDescriptor) -> Self {
        Self {
            text: Text::from_sections(vec![
                TextSection::new(
                    "",
                    TextStyle {
                        font_size: descriptor.font.size,
                        color: Color::rgba(
                            descriptor.font.color.x / 255.0,
                            descriptor.font.color.y / 255.0,
                            descriptor.font.color.z / 255.0,
                            descriptor.font.color.w / 255.0
                        ),
                        // color: Color::GRAY,
                        ..Default::default()
                    },
                );
                4
            ]),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(
                    Val::Px(descriptor.size.x - 10.0),
                    Val::Px(descriptor.size.y - 10.0),
                ),
                position: UiRect {
                    left: Val::Px(descriptor.position.x),
                    top: Val::Px(descriptor.position.y),
                    ..default()
                },
                ..default()
            },
            transform: Transform::from_xyz(descriptor.position.x, descriptor.position.y, 0.0),
            z_index: ZIndex::Global(descriptor.z_index),
            ..default()
        }
    }
}

#[macro_export]
macro_rules! text {
    ($size: expr, $position: expr, $font_size: expr, $color: expr, $z_index: expr) => {{
        TextDescriptor {
            size: Vec2::from($size),
            position: Vec2::from($position),
            font: FontSettings {
                size: $font_size,
                color: Vec4::from($color),
            },
            z_index: $z_index,
            ..Default::default()
        }
    }};
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Component)]
pub struct GroupDescriptor(pub Vec<Descriptor>);

#[test]
fn test_image_serialization() {
    let image = Descriptor {
        id: 0,
        content: WidgetDescriptor::image(image!((100.0, 100.0), (0.0, 0.0), 1)),
        children: None,
    };
    let serialized = serde_json::to_string(&image).unwrap();
    println!("{}", serialized);
}

#[test]
fn test_image_deserialization() {
    let serialized = r#"{"id":0,"image":{"size":[100.0,100.0],"position":[0.0,0.0],"z_index":1}}"#;
    let image = serde_json::from_str::<Descriptor>(serialized).unwrap();
    println!("{:?}", image);
}
