use bevy::prelude::*;

const MAIN_TITLE_BG_IMAGE: &str = "pictures/title_sel0.png";
const START_BUTTON_HOVER_IMAGE: &str = "pictures/button/start_button_hovered.png";
const CONFIG_BUTTON_HOVER_IMAGE: &str = "pictures/button/config_button_hovered.png";
const EXTRA_BUTTON_HOVER_IMAGE: &str = "pictures/button/extra_button_hovered.png";
const EXIT_BUTTON_HOVER_IMAGE: &str = "pictures/button/exit_button_hovered.png";

const CG_BUTTON_HOVER_IMAGE: &str = "pictures/button/cg_button_hovered.png";
const MUSIC_BUTTON_HOVER_IMAGE: &str = "pictures/button/music_button_hovered.png";
const SCENE_BUTTON_HOVER_IMAGE: &str = "pictures/button/scene_button_hovered.png";
const BACK_BUTTON_HOVER_IMAGE: &str = "pictures/button/back_button_hovered.png";

#[derive(Component)]
pub struct UIForStat<T> (pub Vec<T>);

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
    Scene,
    Music,
}

#[derive(Debug)]
pub struct ExtraTitleRes {
    pub cg_button_hover_image: Handle<Image>,
    pub scene_button_hover_image: Handle<Image>,
    pub music_button_hover_image: Handle<Image>,
}

impl ExtraTitleRes {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            cg_button_hover_image: asset_server.load(CG_BUTTON_HOVER_IMAGE),
            scene_button_hover_image: asset_server.load(SCENE_BUTTON_HOVER_IMAGE),
            music_button_hover_image: asset_server.load(MUSIC_BUTTON_HOVER_IMAGE),
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
