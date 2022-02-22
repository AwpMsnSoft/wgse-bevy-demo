use super::descriptors::WidgetId;
use bevy::prelude::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub const MAIN_TITLE_BG_GUID: WidgetId = WidgetId(10100);
pub const START_BUTTON_GUID: WidgetId = WidgetId(10101);
pub const CONFIG_BUTTON_GUID: WidgetId = WidgetId(10102);
pub const EXTRA_BUTTON_GUID: WidgetId = WidgetId(10103);
pub const EXIT_BUTTON_GUID: WidgetId = WidgetId(10104);

lazy_static! {
    pub static ref MAIN_TITLE_RES_MAP: HashMap<WidgetId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(MAIN_TITLE_BG_GUID, "pictures/title_sel0.png"); // main title background
        map.insert(START_BUTTON_GUID, "pictures/button/main_title/start_button_hovered.png"); // start button
        map.insert(CONFIG_BUTTON_GUID, "pictures/button/main_title/config_button_hovered.png"); // config button
        map.insert(EXTRA_BUTTON_GUID, "pictures/button/main_title/extra_button_hovered.png"); // extra button
        map.insert(EXIT_BUTTON_GUID, "pictures/button/main_title/exit_button_hovered.png"); // exit button
        map
    };
}

pub const EXTRA_TITLE_BG_GUID: WidgetId = WidgetId(10200);
pub const CG_BUTTON_GUID: WidgetId = WidgetId(10201);
pub const MUSIC_BUTTON_GUID: WidgetId = WidgetId(10202);
pub const SCENE_BUTTON_GUID: WidgetId = WidgetId(10203);
pub const BACK_BUTTON_GUID: WidgetId = WidgetId(10204);

lazy_static! {
    pub static ref EXTRA_TITLE_RES_MAP: HashMap<WidgetId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(EXTRA_TITLE_BG_GUID, "pictures/title_ex0.png"); // extra title background
        map.insert(CG_BUTTON_GUID, "pictures/button/extra_title/cg_button_hovered.png"); // cg button
        map.insert(MUSIC_BUTTON_GUID, "pictures/button/extra_title/music_button_hovered.png"); // music button
        map.insert(SCENE_BUTTON_GUID, "pictures/button/extra_title/scene_button_hovered.png"); // scene button
        map.insert(BACK_BUTTON_GUID, "pictures/button/extra_title/back_button_hovered.png"); // back button
        map
    };
}

#[derive(Component, Debug)]
pub struct UiImageResources(pub HashMap<&'static str, Handle<Image>>);

impl UiImageResources {
    pub fn new(asset_server: &AssetServer) -> Self {
        debug!("Entering UiImageResources::new(), all images will be loaded.");
        let mut image_map = HashMap::new();
        for (_, &path) in MAIN_TITLE_RES_MAP.iter() {
            image_map.insert(path, asset_server.load(path));
        }
        for (_, &path) in EXTRA_TITLE_RES_MAP.iter() {
            image_map.insert(path, asset_server.load(path));
        }
        UiImageResources(image_map)
    }
}
