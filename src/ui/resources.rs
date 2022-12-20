use crate::ui::descriptors::WidgetId;
use bevy::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

/// The image of each widget is stored in this resource.
#[derive(Component, Debug, Resource)]
pub struct UiImageResources(pub HashMap<&'static str, Handle<Image>>);

impl UiImageResources {
    pub fn new(asset_server: &AssetServer) -> Self {
        debug!("Entering UiImageResources::new(), all images will be loaded.");
        let mut image_map = HashMap::new();
        for res in UI_TITLE_RES_LIST.iter() {
            for (_, &path) in res {
                image_map.insert(path, asset_server.load(path));
            }
            image_map.insert("default.png", asset_server.load("pictures/default.png"));
        }
        UiImageResources(image_map)
    }
}

lazy_static! {
    static ref UI_TITLE_RES_LIST: Vec<HashMap<WidgetId, &'static str>> = vec![
        MAIN_TITLE_RES_MAP.clone(),
        START_TITLE_RES_MAP.clone(),
        EXTRA_TITLE_RES_MAP.clone(),
        CONFIG_SPEED_TITLE_RES_MAP.clone()
    ];
}

pub const MAIN_TITLE_BG_GUID: WidgetId = WidgetId(10100);
pub const MAIN_TITLE_START_BUTTON_GUID: WidgetId = WidgetId(10101);
pub const MAIN_TITLE_CONFIG_BUTTON_GUID: WidgetId = WidgetId(10102);
pub const MAIN_TITLE_EXTRA_BUTTON_GUID: WidgetId = WidgetId(10103);
pub const MAIN_TITLE_EXIT_BUTTON_GUID: WidgetId = WidgetId(10104);

lazy_static! {
    pub static ref MAIN_TITLE_RES_MAP: HashMap<WidgetId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(MAIN_TITLE_BG_GUID, "pictures/title_sel0.png"); // main title background
        map.insert(MAIN_TITLE_START_BUTTON_GUID, "pictures/button/main_title/start_button_hovered.png"); // start button
        map.insert(MAIN_TITLE_CONFIG_BUTTON_GUID, "pictures/button/main_title/config_button_hovered.png"); // config button
        map.insert(MAIN_TITLE_EXTRA_BUTTON_GUID, "pictures/button/main_title/extra_button_hovered.png"); // extra button
        map.insert(MAIN_TITLE_EXIT_BUTTON_GUID, "pictures/button/main_title/exit_button_hovered.png"); // exit button
        map
    };
}

pub const START_TITLE_BG_GUID: WidgetId = WidgetId(10150);
pub const START_TITLE_CG_GUID: WidgetId = WidgetId(10151);
pub const START_TITLE_NAME_TEXTBOX_GUID: WidgetId = WidgetId(10152);
pub const START_TITLE_DIALOG_TEXTBOX_GUID: WidgetId = WidgetId(10153);
pub const START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID: WidgetId = WidgetId(10154);
pub const START_TITLE_FULLSCREEN_DIALOG_TEXTBOX_GUID: WidgetId = WidgetId(10155);
pub const START_TITLE_FULLSCREEN_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID: WidgetId = WidgetId(10156);
pub const START_TITLE_TACHIE1_GUID: WidgetId = WidgetId(10161);
pub const START_TITLE_TACHIE2_GUID: WidgetId = WidgetId(10162);
pub const START_TITLE_TACHIE3_GUID: WidgetId = WidgetId(10163);
pub const START_TITLE_TACHIE4_GUID: WidgetId = WidgetId(10164);
pub const START_TITLE_TACHIE5_GUID: WidgetId = WidgetId(10165);

lazy_static! {
    pub static ref START_TITLE_RES_MAP: HashMap<WidgetId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(START_TITLE_BG_GUID, "pictures/default.png");
        map.insert(START_TITLE_CG_GUID, "pictures/default.png");
        map.insert(START_TITLE_TACHIE1_GUID, "pictures/default.png");
        map.insert(START_TITLE_TACHIE2_GUID, "pictures/default.png");
        map.insert(START_TITLE_TACHIE3_GUID, "pictures/default.png");
        map.insert(START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID, "pictures/default.png"); // dummy button for dialog textbox
        map
    };
}

pub const EXTRA_TITLE_BG_GUID: WidgetId = WidgetId(10200);
pub const EXTRA_TITLE_CG_BUTTON_GUID: WidgetId = WidgetId(10201);
pub const EXTRA_TITLE_SCENE_BUTTON_GUID: WidgetId = WidgetId(10202);
pub const EXTRA_TITLE_MUSIC_BUTTON_GUID: WidgetId = WidgetId(10203);
pub const EXTRA_TITLE_BACK_BUTTON_GUID: WidgetId = WidgetId(10204);

lazy_static! {
    pub static ref EXTRA_TITLE_RES_MAP: HashMap<WidgetId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(EXTRA_TITLE_BG_GUID, "pictures/title_ex0.png"); // extra title background
        map.insert(EXTRA_TITLE_CG_BUTTON_GUID, "pictures/button/extra_title/cg_button_hovered.png"); // cg button
        map.insert(EXTRA_TITLE_MUSIC_BUTTON_GUID, "pictures/button/extra_title/music_button_hovered.png"); // music button
        map.insert(EXTRA_TITLE_SCENE_BUTTON_GUID, "pictures/button/extra_title/scene_button_hovered.png"); // scene button
        map.insert(EXTRA_TITLE_BACK_BUTTON_GUID, "pictures/button/extra_title/back_button_hovered.png"); // back button
        map
    };
}

pub const CONFIG_SPEED_TITLE_BG_GUID: WidgetId = WidgetId(10300);
pub const CONFIG_SPEED_TITLE_SPEED_BUTTON_GUID: WidgetId = WidgetId(10301);
pub const CONFIG_SPEED_TITLE_SOUND_BUTTON_GUID: WidgetId = WidgetId(10302);
pub const CONFIG_SPEED_TITLE_EXTRA_BUTTON_GUID: WidgetId = WidgetId(10303);
pub const CONFIG_SPEED_TITLE_BACK_BUTTON_GUID: WidgetId = WidgetId(10304);
pub const CONFIG_SPEED_TITLE_SKIP_READED_ON_BUTTON_GUID: WidgetId = WidgetId(10305);
pub const CONFIG_SPEED_TITLE_SKIP_READED_OFF_BUTTON_GUID: WidgetId = WidgetId(10306);

lazy_static! {
    pub static ref CONFIG_SPEED_TITLE_RES_MAP: HashMap<WidgetId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(CONFIG_SPEED_TITLE_BG_GUID, "pictures/config_ex0.png"); // config speed title background
        map.insert(CONFIG_SPEED_TITLE_SPEED_BUTTON_GUID, "pictures/button/config_title/speed_button_hovered.png"); // speed button
        map.insert(CONFIG_SPEED_TITLE_SOUND_BUTTON_GUID, "pictures/button/config_title/sound_button_hovered.png"); // sound button
        map.insert(CONFIG_SPEED_TITLE_EXTRA_BUTTON_GUID, "pictures/button/config_title/extra_button_hovered.png"); // extra button
        map.insert(CONFIG_SPEED_TITLE_BACK_BUTTON_GUID, "pictures/button/config_title/back_button_hovered.png"); // back button
        map.insert(CONFIG_SPEED_TITLE_SKIP_READED_ON_BUTTON_GUID, "pictures/button/config_title/skip_on_button_clicked.png"); // skip readed on button
        map.insert(CONFIG_SPEED_TITLE_SKIP_READED_OFF_BUTTON_GUID, "pictures/button/config_title/skip_off_button_clicked.png"); // skip readed off button
        map
    };
}

/// The font used in the game.
#[derive(Component, Debug, Resource)]
pub struct FontResources(pub Handle<Font>);

impl FontResources {
    pub fn new(asset_server: &AssetServer) -> Self {
        FontResources(asset_server.load("fonts/YuanRouHeiTi.ttf"))
        // FontResources(asset_server.load("fonts/Ubuntu-MI.ttf"))
    }
}

lazy_static! {
    /// Panel const table
    pub static ref PANEL_CONST_MAP: HashMap<i32, WidgetId> = {
        let mut map = HashMap::new();
        // Panel for cg, bg and tachie
        map.insert(-1, START_TITLE_BG_GUID); // BG, never changed.
        map.insert(0, START_TITLE_CG_GUID);  // CG
        map.insert(1, START_TITLE_TACHIE1_GUID); // TCHE1
        map.insert(2, START_TITLE_TACHIE2_GUID);
        map.insert(3, START_TITLE_TACHIE3_GUID);
        map.insert(4, START_TITLE_TACHIE4_GUID);
        map.insert(5, START_TITLE_TACHIE5_GUID);
        // Panel for message
        map.insert(10, START_TITLE_DIALOG_TEXTBOX_GUID);
        map.insert(11, START_TITLE_FULLSCREEN_DIALOG_TEXTBOX_GUID);
        map
    };
}
