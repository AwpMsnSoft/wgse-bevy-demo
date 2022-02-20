use bevy::prelude::*;

#[derive(Component)]
pub struct UIForStat(pub Vec<UiState>);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MainTitleState {
    Main,
    Start,
    Config,
    Extra,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StartTitleState {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ConfigTitleState {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ExtraTitleState {
    Cg,
    Scene,
    Music,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UiState {
    MainTitleState(MainTitleState),
    StartTitleState(StartTitleState),
    ConfigTitleState(ConfigTitleState),
    ExtraTitleState(ExtraTitleState),
}

impl From<MainTitleState> for UiState {
    fn from(state: MainTitleState) -> Self {
        UiState::MainTitleState(state)
    }
}

impl From<StartTitleState> for UiState {
    fn from(state: StartTitleState) -> Self {
        UiState::StartTitleState(state)
    }
}

impl From<ConfigTitleState> for UiState {
    fn from(state: ConfigTitleState) -> Self {
        UiState::ConfigTitleState(state)
    }
}

impl From<ExtraTitleState> for UiState {
    fn from(state: ExtraTitleState) -> Self {
        UiState::ExtraTitleState(state)
    }
}
