use super::states::UiState;
use bevy::prelude::*;

#[derive(Debug, Default, Deref, Clone, Component, PartialEq, Eq, Hash)]
pub struct WidgetId(pub i32);

#[derive(Debug, Default, Deref, Clone, Component, PartialEq, Eq, Hash)]
pub struct CString(pub String);

#[derive(Debug, Default, Deref, Clone, Component, PartialEq, Eq, Hash)]
pub struct CUiState(pub Option<UiState>);

#[derive(Debug, Default, Clone, Component, PartialEq, Eq, Hash)]
pub struct UiImage2 {
    pub image0: Handle<Image>,
    pub image1: Handle<Image>,
}

#[generic(Button)]
#[derive(Debug, Default, Bundle)]
pub struct SingleButton {
    pub id: WidgetId,
    pub next_state: CUiState,
    pub images: UiImage2,
}
