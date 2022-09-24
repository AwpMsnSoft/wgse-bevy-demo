use crate::{
    script::commands as wgs,
    text::{
        states::TextRenderingState,
        stepped_iterator::{IntoMonoChars, SteppedIterator},
    },
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

pub fn dialog_textbox_text_trigger(
    ui_state: Res<State<UiState>>,
    mut command: EventReader<wgs::Message>,
    mut text_rendering_state: ResMut<State<TextRenderingState>>,
) {
    if let Some(message) = command.iter().last() {
        if ui_state.current() == &UiState::from(MainTitleState::Start) {
            text_rendering_state
                .set(TextRenderingState::Pendding(message.message.clone()))
                .expect("text_rendering_state.set failed.");
        }
    }
}

pub fn dialog_textbox_text_system(
    mut text_query: Query<(&mut Text, &WidgetId)>,
    ui_state: Res<State<UiState>>,
    font: Res<FontResources>,
    mut text_rendering_state: ResMut<State<TextRenderingState>>,
    mut text_value: Local<String>,
    mut cursor: Local<usize>,
    mut left_bound: Local<usize>,
    mut full_length: Local<usize>,
    time: Res<Time>,
    mut timer: Local<Timer>,
) {
    for (mut text, widget_id) in text_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_DIALOG_TEXTBOX_GUID
        {
            // TODO: Considering remove this clone.
            let current = text_rendering_state.current().clone();
            match current {
                TextRenderingState::Pendding(message) => {
                    *cursor = 0;
                    *left_bound = 0;
                    *text_value = message.clone();
                    *full_length = message.mono_chars().count();
                    *timer = Timer::from_seconds(0.05, true);
                    text_rendering_state
                        .set(TextRenderingState::Rendering)
                        .expect("text_rendering_state.set(TextRenderingState::Rendering) failed.");
                }
                TextRenderingState::Rendering => {
                    timer.tick(time.delta());
                    if timer.finished() {
                        text.sections[0].value = text_value
                            .clone()
                            .mono_chars()
                            .take_stepped(*cursor)
                            .skip_stepped(*left_bound)
                            .collect();
                        text.sections[0].style.font = font.0.clone();
                        // TODO: Magic number
                        *cursor += 1;
                        if *cursor - *left_bound >= 2 * 23 * 3 {
                            *left_bound += 2 * 23;
                        }
                        if *cursor > *full_length {
                            text_rendering_state.set(TextRenderingState::Done).expect(
                                "text_rendering_state.set(TextRenderingState::Done) failed.",
                            );
                        }
                    }
                }
                TextRenderingState::Done => (),
            }
        }
    }
}