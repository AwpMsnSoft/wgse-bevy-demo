use super::states::UiState;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Component, PartialEq, Eq, Hash)]
pub struct WidgetId(pub i32);

/// A button widget which:
///     - unvisiable when `Interaction::None`
///     - visible when `Interaction::Hover`
///     - turning to `next_state` when `Interaction::Click`
#[generic(Button)]
#[derive(Bundle, Clone, Debug)]
pub struct SingleButton {
    /// the target state when clicked
    pub next_state: UiState,
}