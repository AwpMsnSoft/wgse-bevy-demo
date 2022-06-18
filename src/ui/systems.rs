use crate::ui::{
    components::{UiFloat, UiImageSet},
    policies::*,
};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::ui::Interaction;

pub fn hoverable_policy_system(
    mut policy_query: Query<
        (&UiImageSet, &mut UiImage, &Interaction),
        (With<HoverablePolicy>, Changed<Interaction>),
    >,
) {
    for (ui_image_set, mut ui_iamge, interaction) in policy_query.iter_mut() {
        match interaction {
            Interaction::None => {
                if let Some(image0) = ui_image_set.image0.as_ref() {
                    *ui_iamge = UiImage(image0.clone_weak());
                }
            }
            Interaction::Hovered => {
                if let Some(image1) = ui_image_set.image1.as_ref() {
                    *ui_iamge = UiImage(image1.clone_weak());
                }
            }
            Interaction::Clicked => {
                if let Some(image2) = ui_image_set.image2.as_ref() {
                    *ui_iamge = UiImage(image2.clone_weak());
                }
            }
        }
    }
}

/// Change the UiImage size to simulate a `slide` effect.
pub fn slidable_policy_system(
    mut policy_query: Query<(&mut UiFloat, &mut Style, &Interaction), With<SlidablePolicy>>,
    mut mouse: EventReader<CursorMoved>,
) {
    for (mut cursor, mut ui_style, interaction) in policy_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                for mouse in mouse.iter() {
                    // Get current mouse position (Window coordinates) and button size.
                    let (cur_x, _) = mouse.position.into();
                    let root_x = match ui_style.position.left {
                        Val::Px(x) => x,
                        _ => 0.,
                    };
                    let width = match ui_style.size.width {
                        Val::Px(w) => w,
                        _ => 0.,
                    };
                    // Map the cursor position to [0, 1].
                    cursor.0 = (cur_x - root_x) / width;
                    // Reset button's UiImage Size.
                    ui_style.size.width = Val::Px(width * cursor.0);
                    ui_style.position.right = Val::Px(root_x + width * cursor.0);
                }
            }
            _ => (),
        }
    }
}
