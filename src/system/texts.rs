use crate::ui::{
    descriptors::{Descriptor, TextDescriptor},
    resources::FontResources,
    states::{StartTitleState, UiState},
};
use bevy::prelude::*;

pub fn text_spawn_system_curied(
    layout: &'static Vec<Descriptor>,
) -> impl Fn(Commands, Res<State<UiState>>) {
    move |command, ui_state| {
        debug!("Spawning title. current state: {:?}", ui_state);
    }
}
