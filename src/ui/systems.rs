use crate::ui::{
    components::*,
    policies::*,
};
use bevy::prelude::*;
use bevy::ui::Interaction;
use rayon::prelude::*;

/// Show different image based on the button interaction.
pub fn hoverable_policy_system(
    mut policy_query: Query<
        (&UiImageSet, &mut UiImage, &Interaction),
        (With<HoverablePolicy>, With<Button>, Changed<Interaction>),
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

/// Change the UiImage alpha channel to simulate a `slide` effect.
pub fn slidable_policy_system(
    mut policy_query: Query<
        (&mut UiFloat, &UiImage, &Style, &Interaction),
        (With<SlidablePolicy>, With<Button>),
    >,
    mut mouse: EventReader<CursorMoved>,
    mut images: ResMut<Assets<Image>>,
) {
    for (mut cursor, ui_image, ui_style, interaction) in policy_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                for mouse in mouse.iter() {
                    // [=========================--------------]
                    //                          ^
                    //                          | cursor_pos = mouse_pos.x - button_pos.left
                    //                          | cursor = cursor_pos / button_size.width
                    let cursor_pos = mouse.position.x
                        - match ui_style.position.left {
                            Val::Px(val) => val,
                            _ => unreachable!(),
                        };
                    let button_width = match ui_style.size.width {
                        Val::Px(val) => val,
                        _ => unreachable!(),
                    };
                    cursor.0 = (cursor_pos / button_width).clamp(0.0, 1.0);
                    // [=========================--------------]
                    //                          ^
                    //                          | set the image alpha channel depending on the cursor position
                    //                          | alpha_left = 0xFF, alpha_right = 0x00
                    let image = images.get_mut(&ui_image.0).unwrap();
                    let bound = (image.size().x * cursor.0) as usize;
                    // bevy stores images in RGBA format
                    // alpha byte start at index 3, step by 4 bytes
                    image
                        .data
                        .par_iter_mut()
                        .enumerate()
                        .skip(3)
                        .step_by(4)
                        .for_each(|(idx, ch)| {
                            *ch = if idx < bound { 0xFF } else { 0x00 };
                        });
                }
            }
            _ => (),
        }
    }
}

/// Change active child of a `Switch` based on the button interaction.
pub fn switchable_policy_system(
    mut policy_query: Query<
        (&UiEntity, &Children),
        (With<SwitchablePolicy>, Changed<UiEntity>),
    >,
    mut children_query: Query<&mut Visibility>,
) {
    for (active_child, children) in policy_query.iter() {
        if !children.contains(active_child) {
            panic!("`active_child` {:?} is not a child of the `Switch`, maybe a memory overlap?", active_child);
        }
        // Hide all children
        for mut vis in children_query.iter_mut() {
            vis.is_visible = false;
        }
        // Then show the active one.
        let mut active_child_vis = children_query.get_mut(active_child.0).unwrap();
        active_child_vis.is_visible = true;
    }
}

/// Set parent's `active_child` filed to the child that is clicked.
pub fn sub_switchable_policy_system(
    mut policy_query: Query<
        (Entity, &Interaction, &Parent),
        (With<SubSwitchablePolicy>, With<Button>, Changed<Interaction>),
    >,
    mut parent_query: Query<&mut UiEntity>,
) {
    for (entity, interaction, parent) in policy_query.iter() {
        if let Interaction::Clicked = interaction {
            let mut parent_active_child = parent_query.get_mut(parent.0).unwrap();
            *parent_active_child = UiEntity(entity);
        }
    }
}