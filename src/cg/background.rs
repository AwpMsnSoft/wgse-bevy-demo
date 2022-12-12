use crate::{
    script::commands as wgs,
    ui::{
        descriptors::WidgetId,
        resources::START_TITLE_CG_GUID,
        states::{MainTitleState, UiState},
    },
};
use bevy::prelude::*;

pub fn background_display_system(
    mut background_query: Query<(&mut UiImage, &WidgetId)>,
    ui_state: Res<State<UiState>>,
    mut background_image: EventReader<wgs::Cg>,
    asset_server: Res<AssetServer>,
    mut trigger: EventWriter<wgs::Next>,
) {
    for (mut image, widget_id) in background_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_CG_GUID
        {
            if let Some(cg) = background_image.iter().last() {
                *image = asset_server
                    .load(format!("pictures/image/{}.png", cg.path))
                    .into();
                trigger.send(wgs::Next {});
            }
        }
    }
}
