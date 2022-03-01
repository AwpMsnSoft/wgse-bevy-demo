use crate::ui::{
    descriptors::WidgetId,
    resources::*,
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
    // Config,
    Extra,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StartTitleState {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ConfigTitleState {
    Speed,
    Sound,
    Extra,
}

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
    pub static ref MAIN_TITLE_BUTTON_STATE_MAP: HashMap<WidgetId, UiButtonState> = {
        let mut map = HashMap::new();
        map.insert(
            MAIN_TITLE_START_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(MainTitleState::Start.into()),
            )),
        );
        map.insert(
            MAIN_TITLE_CONFIG_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(ConfigTitleState::Speed.into()),
            )),
        );
        map.insert(
            MAIN_TITLE_EXTRA_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Main.into(),
                Some(MainTitleState::Extra.into()),
            )),
        );
        map.insert(
            MAIN_TITLE_EXIT_BUTTON_GUID,
            UiButtonState::from((MainTitleState::Main.into(), None)),
        );
        map
    };
}

lazy_static! {
    pub static ref EXTRA_TITLE_BUTTON_STATE_MAP: HashMap<WidgetId, UiButtonState> = {
        let mut map = HashMap::new();
        map.insert(
            EXTRA_TITLE_MUSIC_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Music.into()),
            )),
        );
        map.insert(
            EXTRA_TITLE_SCENE_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Scene.into()),
            )),
        );
        map.insert(
            EXTRA_TITLE_CG_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(ExtraTitleState::Cg.into()),
            )),
        );
        map.insert(
            EXTRA_TITLE_BACK_BUTTON_GUID,
            UiButtonState::from((
                MainTitleState::Extra.into(),
                Some(MainTitleState::Main.into()),
            )),
        );
        map
    };
}

lazy_static! {
    pub static ref CONFIG_SPEED_TITLE_BUTTON_STATE_MAP: HashMap<WidgetId, UiButtonState> = {
        let mut map = HashMap::new();
        map.insert(
            CONFIG_SPEED_TITLE_SPEED_BUTTON_GUID,
            UiButtonState::from((ConfigTitleState::Speed.into(), None)),
        );
        map.insert(
            CONFIG_SPEED_TITLE_SOUND_BUTTON_GUID,
            UiButtonState::from((
                ConfigTitleState::Speed.into(),
                Some(ConfigTitleState::Sound.into()),
            )),
        );
        map.insert(
            CONFIG_SPEED_TITLE_EXTRA_BUTTON_GUID,
            UiButtonState::from((
                ConfigTitleState::Speed.into(),
                Some(ConfigTitleState::Extra.into()),
            )),
        );
        map.insert(
            CONFIG_SPEED_TITLE_BACK_BUTTON_GUID,
            UiButtonState::from((
                ConfigTitleState::Speed.into(),
                Some(MainTitleState::Main.into()),
            )),
        );
        map.insert(
            CONFIG_SPEED_TITLE_SKIP_READED_ON_BUTTON_GUID,
            UiButtonState::from((
                ConfigTitleState::Speed.into(),
                None
            ))
        );
        map.insert(
            CONFIG_SPEED_TITLE_SKIP_READED_OFF_BUTTON_GUID,
            UiButtonState::from((
                ConfigTitleState::Speed.into(),
                None
            ))
        );
        map
    };
}
