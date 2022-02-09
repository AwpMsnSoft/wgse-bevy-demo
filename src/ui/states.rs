use bevy::prelude::*;

const MAIN_TITLE_BG_IMAGE: &str = "textures/title_sel0_2x.png";
const START_BUTTON_HOVER_IMAGE: &str = "textures/title_start_button_hovered.png";
const CONFIG_BUTTON_HOVER_IMAGE: &str = "textures/title_config_button_hovered.png";
const EXTRA_BUTTON_HOVER_IMAGE: &str = "textures/title_extra_button_hovered.png";
const EXIT_BUTTON_HOVER_IMAGE: &str = "textures/title_exit_button_hovered.png";

const CG_BUTTON_HOVER_IMAGE: &str = "textures/title_cg_button_hovered.png";
const MUSIC_BUTTON_HOVER_IMAGE: &str = "textures/title_music_button_hovered.png";
const EVENT_BUTTON_HOVER_IMAGE: &str = "textures/title_event_button_hovered.png";

#[derive(Component)]
pub struct UIForStat<T> (Vec<T>);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MainTitleState {
    Main,
    Start,
    Config,
    Extra,
}

#[derive(Debug)]
pub struct MainTitleRes {
    pub bg_image: Handle<Image>,
    pub start_button_hover_image: Handle<Image>,
    pub config_button_hover_image: Handle<Image>,
    pub extra_button_hover_image: Handle<Image>,
    pub exit_button_hover_image: Handle<Image>,
}

impl MainTitleRes {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            bg_image: asset_server.load(MAIN_TITLE_BG_IMAGE),
            start_button_hover_image: asset_server.load(START_BUTTON_HOVER_IMAGE),
            config_button_hover_image: asset_server.load(CONFIG_BUTTON_HOVER_IMAGE),
            extra_button_hover_image: asset_server.load(EXTRA_BUTTON_HOVER_IMAGE),
            exit_button_hover_image: asset_server.load(EXIT_BUTTON_HOVER_IMAGE),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ExtraTitleState {
    Cg,
    Music,
    Event,
}

#[derive(Debug)]
pub struct ExtraTitleRes {
    pub cg_image: Handle<Image>,
    pub music_image: Handle<Image>,
    pub event_image: Handle<Image>,
}

impl ExtraTitleRes {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            cg_image: asset_server.load(CG_BUTTON_HOVER_IMAGE),
            music_image: asset_server.load(MUSIC_BUTTON_HOVER_IMAGE),
            event_image: asset_server.load(EVENT_BUTTON_HOVER_IMAGE),
        }
    }
}

pub fn main_title_despawn(
    mut commands: Commands,
    state: Res<State<MainTitleState>>,
    query: Query<(Entity, &UIForStat<MainTitleState>)>,
) {
    for (entity, states) in &mut query.iter() {
        if !states.0.contains(state.current()) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn extra_title_despawn(
    mut commands: Commands,
    state: Res<State<ExtraTitleState>>,
    query: Query<(Entity, &UIForStat<ExtraTitleState>)>,
) {
    for (entity, states) in &mut query.iter() {
        if !states.0.contains(state.current()) {
            commands.entity(entity).despawn();
        }
    }
}
