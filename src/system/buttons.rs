use crate::ui::{resources::CONFIG_BUTTON_GUID, widget::WidgetID};
use bevy::prelude::*;

pub fn config_button_event(
    mut interaction_query: Query<
        (&Interaction, &mut Visibility, &WidgetID),
        (Changed<Interaction>, With<WidgetID>),
    >,
) {
    for (&interaction, mut visibility, id) in interaction_query.iter_mut() {
        if id.0 == CONFIG_BUTTON_GUID {
            match interaction {
                Interaction::Hovered => {
                    info!("ui/title Config button hovered.");
                    (*visibility).is_visible = true;
                }
                _ => {
                    (*visibility).is_visible = false;
                }
            }
        }
    }
}
