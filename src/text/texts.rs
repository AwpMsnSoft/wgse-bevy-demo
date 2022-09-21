use crate::{
    script::commands as wgs,
    text::stepped_iterator::{IntoSteppedIter, SteppedIterator},
    ui::{
        descriptors::WidgetId,
        resources::{
            FontResources, START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID,
            START_TITLE_DIALOG_TEXTBOX_GUID, START_TITLE_NAME_TEXTBOX_GUID,
        },
        states::{MainTitleState, UiState},
    },
};
use bevy::prelude::*;

pub fn dialog_textbox_button_system(
    mut interaction_query: Query<(&Interaction, &WidgetId), Changed<Interaction>>,
    ui_state: Res<State<UiState>>,
    mut next: EventWriter<wgs::Next>,
) {
    for (interaction, widget_id) in interaction_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID
        {
            if interaction == &Interaction::Clicked {
                // Get next dialog from script
                debug!("Getting next dialog from script...");
                next.send(wgs::Next {});
            }
        }
    }
}

pub fn dialog_textbox_name_system(
    mut text_query: Query<(&mut Text, &WidgetId)>,
    ui_state: Res<State<UiState>>,
    font: Res<FontResources>,
    mut command: EventReader<wgs::Message>,
) {
    for (mut text, widget_id) in text_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_NAME_TEXTBOX_GUID
        {
            if let Some(message) = command.iter().last() {
                text.sections[0].value = message.chara.clone();
                text.sections[0].style.font = font.0.clone();
                text.alignment = TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                };
            }
        }
    }
}

pub fn dialog_textbox_text_system(
    mut text_query: Query<(&mut Text, &WidgetId)>,
    ui_state: Res<State<UiState>>,
    font: Res<FontResources>,
    mut command: EventReader<wgs::Message>,
    mut timer: Local<Option<Timer>>,
    mut cursor: Local<usize>,
    time: Res<Time>,
) {
    if let Some(timer) = &mut *timer {
        for (mut text, widget_id) in text_query.iter_mut() {
            if ui_state.current() == &UiState::from(MainTitleState::Start)
                && widget_id == &START_TITLE_DIALOG_TEXTBOX_GUID
            {
                if timer.finished() {
                    *cursor += 1;
                }
                if let Some(message) = command.iter().last() {
                    text.sections[0].value = message
                        .message
                        .clone()
                        .chars()
                        .into_stepped_iter(|ch| if ch.is_ascii() { 1 } else { 2 })
                        .take_stepped(*cursor)
                        .skip_stepped(23)
                        .collect();
                    text.sections[0].style.font = font.0.clone();
                }
                timer.tick(time.delta());
            }
        }
    } else {
        *timer = Some(Timer::from_seconds(0.16, true));
    }
}
