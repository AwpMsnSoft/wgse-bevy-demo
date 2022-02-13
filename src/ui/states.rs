use bevy::prelude::*;

#[derive(Component)]
pub struct UIForStat(pub Vec<UIState>);

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
pub enum UIState {
    MainTitleState(MainTitleState),
    StartTitleState(StartTitleState),
    ConfigTitleState(ConfigTitleState),
    ExtraTitleState(ExtraTitleState),
}

impl From<MainTitleState> for UIState {
    fn from(state: MainTitleState) -> Self {
        UIState::MainTitleState(state)
    }
}

impl From<StartTitleState> for UIState {
    fn from(state: StartTitleState) -> Self {
        UIState::StartTitleState(state)
    }
}

impl From<ConfigTitleState> for UIState {
    fn from(state: ConfigTitleState) -> Self {
        UIState::ConfigTitleState(state)
    }
}

impl From<ExtraTitleState> for UIState {
    fn from(state: ExtraTitleState) -> Self {
        UIState::ExtraTitleState(state)
    }
}
