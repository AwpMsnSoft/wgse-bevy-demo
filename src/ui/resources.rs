use bevy::prelude::*;

pub const MAIN_TITLE_BG_IMAGE: &str = "pictures/title_sel0.png";
pub const START_BUTTON_HOVER_IMAGE: &str = "pictures/button/start_button_hovered.png";
pub const CONFIG_BUTTON_HOVER_IMAGE: &str = "pictures/button/config_button_hovered.png";
pub const EXTRA_BUTTON_HOVER_IMAGE: &str = "pictures/button/extra_button_hovered.png";
pub const EXIT_BUTTON_HOVER_IMAGE: &str = "pictures/button/exit_button_hovered.png";

pub const CONFIG_BUTTON_GUID: i32 = 10001;

pub const EXTRA_TILTE_BG_IAMGE: &str = "pictures/title_ex0.png";
pub const CG_BUTTON_HOVER_IMAGE: &str = "pictures/button/cg_button_hovered.png";
pub const MUSIC_BUTTON_HOVER_IMAGE: &str = "pictures/button/music_button_hovered.png";
pub const SCENE_BUTTON_HOVER_IMAGE: &str = "pictures/button/scene_button_hovered.png";
pub const BACK_BUTTON_HOVER_IMAGE: &str = "pictures/button/back_button_hovered.png";

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
        info!("ui/title resources loaded.");
        Self {
            bg_image: asset_server.load(MAIN_TITLE_BG_IMAGE),
            start_button_hover_image: asset_server.load(START_BUTTON_HOVER_IMAGE),
            config_button_hover_image: asset_server.load(CONFIG_BUTTON_HOVER_IMAGE),
            extra_button_hover_image: asset_server.load(EXTRA_BUTTON_HOVER_IMAGE),
            exit_button_hover_image: asset_server.load(EXIT_BUTTON_HOVER_IMAGE),
        }
    }
}

#[derive(Debug)]
pub struct ExtraTitleRes {
    pub bg_image: Handle<Image>,
    pub cg_button_hover_image: Handle<Image>,
    pub scene_button_hover_image: Handle<Image>,
    pub music_button_hover_image: Handle<Image>,
    pub back_button_hover_image: Handle<Image>,
}

impl ExtraTitleRes {
    pub fn new(asset_server: &AssetServer) -> Self {
        info!("ui/extra resources loaded.");
        Self {
            bg_image: asset_server.load(EXTRA_TILTE_BG_IAMGE),
            cg_button_hover_image: asset_server.load(CG_BUTTON_HOVER_IMAGE),
            scene_button_hover_image: asset_server.load(SCENE_BUTTON_HOVER_IMAGE),
            music_button_hover_image: asset_server.load(MUSIC_BUTTON_HOVER_IMAGE),
            back_button_hover_image: asset_server.load(BACK_BUTTON_HOVER_IMAGE),
        }
    }
}