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

impl Into<UIState> for MainTitleState {
    fn into(self) -> UIState {
        UIState::MainTitleState(self)
    }
}

impl Into<UIState> for StartTitleState {
    fn into(self) -> UIState {
        UIState::StartTitleState(self)
    }
}

impl Into<UIState> for ConfigTitleState {
    fn into(self) -> UIState {
        UIState::ConfigTitleState(self)
    }
}

impl Into<UIState> for ExtraTitleState {
    fn into(self) -> UIState {
        UIState::ExtraTitleState(self)
    }
}
