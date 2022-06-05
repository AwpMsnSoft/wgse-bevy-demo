use crate::ui::{components::UiImageSet, policies::*};
use bevy::prelude::*;
use bevy::ui::Interaction;

pub fn hovering_policy_system(
    mut policy_query: Query<
        (&UiImageSet, &mut UiImage, &Interaction),
        (With<HoveringPolicy>, Changed<Interaction>),
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
