use crate::{
    script::{commands as wgs, runtime::WgsContext},
    ui::{
        descriptors::WidgetId,
        resources::PANEL_CONST_MAP,
        states::{MainTitleState, UiState},
    },
};
use bevy::prelude::*;

pub fn image_display_system(
    mut image_query: Query<(&mut UiImage, &WidgetId)>,
    mut wgs_context: Query<&WgsContext>,
    ui_state: Res<State<UiState>>,
    mut image: EventReader<wgs::Image>,
    asset_server: Res<AssetServer>,
    mut trigger: EventWriter<wgs::Next>,
) {
    for (mut old_image, widget_id) in image_query.iter_mut() {
        let context = wgs_context.single_mut();
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == (*PANEL_CONST_MAP).get(&context.registers.pn).unwrap()
        {
            if let Some(image) = image.iter().last() {
                *old_image = asset_server
                    .load(format!("pictures/image/{}.png", image.path))
                    .into();
                trigger.send(wgs::Next {});
            }
        }
    }
}
