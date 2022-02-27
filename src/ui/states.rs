use crate::ui::{
    descriptors::WidgetId,
    resources::{
        BACK_BUTTON_GUID, CG_BUTTON_GUID, CONFIG_BUTTON_GUID, EXIT_BUTTON_GUID, EXTRA_BUTTON_GUID,
        MUSIC_BUTTON_GUID, SCENE_BUTTON_GUID, START_BUTTON_GUID,
    },
};
use bevy::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub struct UiButtonState {
    pub current: UiState,
    pub turning_into: Option<UiState>,
}

impl From<(UiState, Option<UiState>)> for UiButtonState {
    fn from(state: (UiState, Option<UiState>)) -> Self {
        UiButtonState {
            current: state.0,
            turning_into: state.1,
        }
    }
}

lazy_static! {
    pub static ref TITLE_BUTTON_STATE_LIST: Vec<HashMap<WidgetId, UiButtonState>> = vec![
        MAIN_TITLE_BUTTON_STATE_MAP.clone(),
        EXTRA_TITLE_BUTTON_STATE_MAP.clone()
    ];
}

lazy_static! {
    static ref MAIN_TITLE_BUTTON_STATE_MAP: HashMap<WidgetId, UiButtonState> = {
        let mut map = HashMap::new();
        map.insert(
            START_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(MainTitleState::Start.into()),
            )),
        );
        map.insert(
            CONFIG_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(MainTitleState::Config.into()),
            )),
        );
        map.insert(
            EXTRA_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(MainTitleState::Extra.into()),
            )),
        );
        map.insert(
            EXIT_BUTTON_GUID,
            UiButtonState::from((MainTitleState::Main.into(), None)),
        );
        map
    };
}

lazy_static! {
    static ref EXTRA_TITLE_BUTTON_STATE_MAP: HashMap<WidgetId, UiButtonState> = {
        let mut map = HashMap::new();
        map.insert(
            MUSIC_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Music.into()),
            )),
        );
        map.insert(
            SCENE_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Scene.into()),
            )),
        );
        map.insert(
            CG_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Cg.into()),
            )),
        );
        map.insert(
            BACK_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(MainTitleState::Main.into()),
            )),
        );
        map
    };
}
