use super::descriptors::WidgetID;
use bevy::prelude::*;
use std::collections::HashMap;

pub const MAIN_TITLE_BG_IMAGE: &str = "pictures/title_sel0.png";
pub const START_BUTTON_HOVER_IMAGE: &str = "pictures/button/start_button_hovered.png";
pub const CONFIG_BUTTON_HOVER_IMAGE: &str = "pictures/button/config_button_hovered.png";
pub const EXTRA_BUTTON_HOVER_IMAGE: &str = "pictures/button/extra_button_hovered.png";
pub const EXIT_BUTTON_HOVER_IMAGE: &str = "pictures/button/exit_button_hovered.png";

pub const MAIN_TITLE_BG_GUID: WidgetID = WidgetID(10100);
pub const START_BUTTON_GUID: WidgetID = WidgetID(10101);
pub const CONFIG_BUTTON_GUID: WidgetID = WidgetID(10102);
pub const EXTRA_BUTTON_GUID: WidgetID = WidgetID(10103);
pub const EXIT_BUTTON_GUID: WidgetID = WidgetID(10104);

pub const EXTRA_TILTE_BG_IAMGE: &str = "pictures/title_ex0.png";
pub const CG_BUTTON_HOVER_IMAGE: &str = "pictures/button/cg_button_hovered.png";
pub const MUSIC_BUTTON_HOVER_IMAGE: &str = "pictures/button/music_button_hovered.png";
pub const SCENE_BUTTON_HOVER_IMAGE: &str = "pictures/button/scene_button_hovered.png";
pub const BACK_BUTTON_HOVER_IMAGE: &str = "pictures/button/back_button_hovered.png";

pub const EXTRA_TITLE_BG_GUID: WidgetID = WidgetID(10200);
pub const CG_BUTTON_GUID: WidgetID = WidgetID(10201);
pub const MUSIC_BUTTON_GUID: WidgetID = WidgetID(10202);
pub const SCENE_BUTTON_GUID: WidgetID = WidgetID(10203);
pub const BACK_BUTTON_GUID: WidgetID = WidgetID(10204);

#[derive(Component, Debug)]
pub struct UiImageResources(pub HashMap<WidgetID, Handle<Image>>);

impl UiImageResources {
    pub fn new(asset_server: &AssetServer) -> Self {
        debug!("Entering UiImageResources::new(), all images will be loaded.");
        let mut image_map = HashMap::new();
        image_map.insert(MAIN_TITLE_BG_GUID, asset_server.load(MAIN_TITLE_BG_IMAGE));
        image_map.insert(
            START_BUTTON_GUID,
            asset_server.load(START_BUTTON_HOVER_IMAGE),
        );
        image_map.insert(
            CONFIG_BUTTON_GUID,
            asset_server.load(CONFIG_BUTTON_HOVER_IMAGE),
        );
        image_map.insert(
            EXTRA_BUTTON_GUID,
            asset_server.load(EXTRA_BUTTON_HOVER_IMAGE),
        );
        image_map.insert(EXIT_BUTTON_GUID, asset_server.load(EXIT_BUTTON_HOVER_IMAGE));
        image_map.insert(EXTRA_TITLE_BG_GUID, asset_server.load(EXTRA_TILTE_BG_IAMGE));
        image_map.insert(CG_BUTTON_GUID, asset_server.load(CG_BUTTON_HOVER_IMAGE));
        image_map.insert(
            MUSIC_BUTTON_GUID,
            asset_server.load(MUSIC_BUTTON_HOVER_IMAGE),
        );
        image_map.insert(
            SCENE_BUTTON_GUID,
            asset_server.load(SCENE_BUTTON_HOVER_IMAGE),
        );
        image_map.insert(BACK_BUTTON_GUID, asset_server.load(BACK_BUTTON_HOVER_IMAGE));
        UiImageResources(image_map)
    }
}
