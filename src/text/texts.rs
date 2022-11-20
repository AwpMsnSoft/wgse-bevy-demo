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
    mut text_rendering_state: ResMut<State<TextRenderingState>>,
    mut next: EventWriter<wgs::Next>,
) {
    for (interaction, widget_id) in interaction_query.iter_mut() {
        if ui_state.current() == &UiState::from(MainTitleState::Start)
            && widget_id == &START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID
        {
            if interaction == &Interaction::Clicked {
                let current = text_rendering_state.current().clone();
                match current {
                    TextRenderingState::Pendding(_) => {
                        unreachable!("Double click in one frame will NEVER happend.")
                    }
                    TextRenderingState::Rendering => {
                        text_rendering_state
                            .set(TextRenderingState::Done)
                            .expect("text_rendering_state.set(TextRenderingState::Done) failed.");
                    }
                    TextRenderingState::Done => {
                        // Get next dialog from script
                        debug!("Getting next dialog from script...");
                        next.send(wgs::Next {});
                    }
                }
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
                // Dummy
                text.sections[1].value = String::new();
                text.sections[2].value = String::new();
                text.sections[3].value = String::new();
                text.sections[0].style.font = font.0.clone();
                text.sections[1].style.font = font.0.clone();
                text.sections[2].style.font = font.0.clone();
                text.sections[3].style.font = font.0.clone();
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
            let current = text_rendering_state.current().clone();
            text_rendering_state
                .set(match current {
                    TextRenderingState::Pendding(_) => {
                        unreachable!("Double click in one frame will NEVER happend.")
                    }
                    TextRenderingState::Rendering => TextRenderingState::Done,
                    TextRenderingState::Done => {
                        TextRenderingState::Pendding(message.message.clone())
                    }
                })
                .expect(&format!(
                    "Setting rendering state from {:?} failed.",
                    current
                ));
        }
    }
}

pub fn dialog_textbox_text_system(
    mut text_query: Query<(&mut Text, &WidgetId)>,
    ui_state: Res<State<UiState>>,
    font: Res<FontResources>,
    mut text_rendering_state: ResMut<State<TextRenderingState>>,
    mut splited_message: Local<Vec<String>>,
    mut splited_point: Local<Vec<usize>>,
    mut cursor: Local<usize>,
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
                    // Split message
                    splited_message
                        .set(Box::new(
                            message
                                .mono_chars()
                                .fold((vec![String::new()], 0), |(mut splited, cursor), ch| {
                                    if cursor >= 41 * splited.len() {
                                        splited.push(String::new());
                                    }
                                    *splited.last_mut().unwrap() += &ch.to_string();
                                    (splited, cursor + if ch.is_ascii() { 1 } else { 2 })
                                })
                                .0
                                .into_iter()
                                .map(|s| s + "\n")
                                .collect::<Vec<_>>(),
                        ))
                        .expect(&format!("Split message {} failed.", message));
                    // Cumsum split point
                    splited_point
                        .set(Box::new(splited_message.iter().fold(
                            vec![0],
                            |mut acc, str| {
                                acc.push(acc.last().unwrap() + str.mono_chars().count());
                                acc
                            },
                        )))
                        .expect(&format!("Split message {} failed.", message));
                    // Reset timer and cursor
                    *cursor = 0;
                    *timer = Timer::from_seconds(0.016, TimerMode::Repeating);
                    // Pre-fill all text section
                    (0..4).for_each(|i| {
                        text.sections[i].value = String::new();
                        text.sections[i].style.font = font.0.clone();
                    });
                    // Switch to Rendering state
                    text_rendering_state
                        .set(TextRenderingState::Rendering)
                        .expect("text_rendering_state.set(TextRenderingState::Rendering) failed.");
                }
                TextRenderingState::Rendering => {
                    timer.tick(time.delta());
                    if timer.finished() {
                        // Find curr line
                        let line = splited_point
                            .binary_search(&*cursor)
                            .unwrap_or_else(|u| u - 1);
                        // TODO: Waiting for refactor
                        match line {
                            0 => {
                                text.sections[0].value = splited_message[0]
                                    .mono_chars()
                                    .take_stepped(*cursor)
                                    .collect();
                            }
                            1 => {
                                text.sections[0].value = splited_message[0].clone();
                                text.sections[1].value = splited_message[1]
                                    .mono_chars()
                                    .take_stepped(*cursor - splited_point[1])
                                    .collect();
                            }
                            i @ _ => {
                                text.sections[0].value = splited_message[i - 2].clone();
                                text.sections[1].value = splited_message[i - 1].clone();
                                text.sections[2].value = splited_message[i]
                                    .mono_chars()
                                    .take_stepped(*cursor - splited_point[i])
                                    .collect();
                            }
                        }
                        // Inc cursor
                        *cursor += 1;
                        if *cursor >= *splited_point.last().unwrap() {
                            text_rendering_state.set(TextRenderingState::Done).expect(
                                "text_rendering_state.set(TextRenderingState::Done) failed.",
                            );
                        }
                    }
                }
                TextRenderingState::Done => {
                    let line = splited_message.len();
                    (0..3.min(line)).for_each(|i| {
                        text.sections[i].value = splited_message[line - 3.min(line) + i].clone();
                    });
                }
            }
        }
    }
}
