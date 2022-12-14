use crate::{
    button, descriptor, image, text,
    ui::{
        descriptors::{
            ButtonDescriptor, Descriptor, FontSettings, GroupDescriptor, ImageDescriptor,
            TextDescriptor, WidgetDescriptor,
        },
        resources::*,
    },
};
use bevy::prelude::*;
use lazy_static::lazy_static;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

lazy_static! {
    pub static ref MAIN_TITLE_LAYOUT: Vec<Descriptor> = vec![descriptor!(
        MAIN_TITLE_BG_GUID.0,
        WidgetDescriptor::image(image!((WINDOW_WIDTH, WINDOW_HEIGHT), (0.0, 0.0), 0)),
        Some(GroupDescriptor(vec![
            descriptor!(
                MAIN_TITLE_START_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((100.0, 40.0), (200.0, 480.0), 1)),
                None
            ),
            descriptor!(
                MAIN_TITLE_CONFIG_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((100.0, 40.0), (300.0, 480.0), 1)),
                None
            ),
            descriptor!(
                MAIN_TITLE_EXTRA_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((100.0, 40.0), (400.0, 480.0), 1)),
                None
            ),
            descriptor!(
                MAIN_TITLE_EXIT_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((100.0, 40.0), (500.0, 480.0), 1)),
                None
            )
        ]))
    ),];
}

lazy_static! {
    pub static ref START_TITLE_LAYOUT: Vec<Descriptor> = vec![descriptor!(
        START_TITLE_BG_GUID.0,
        WidgetDescriptor::image(image!((WINDOW_WIDTH, WINDOW_HEIGHT), (0.0, 0.0), 2)),
        Some(GroupDescriptor(vec![
            // background
            descriptor!(
                START_TITLE_CG_GUID.0,
                WidgetDescriptor::image(image!(
                    (WINDOW_WIDTH, WINDOW_HEIGHT), (0.0, 0.0), 0
                )),
                None
            ),
            // chara
            descriptor!(
                START_TITLE_TACHIE1_GUID.0,
                WidgetDescriptor::image(image!(
                    (600.0, 600.0), (100.0, 0.0), 1
                )),
                None
            ),
            descriptor!(
                START_TITLE_TACHIE2_GUID.0,
                WidgetDescriptor::image(image!(
                    (600.0, 600.0), (-50.0, 0.0), 1
                )),
                None
            ),
            descriptor!(
                START_TITLE_TACHIE3_GUID.0,
                WidgetDescriptor::image(image!(
                    (600.0, 600.0), (250.0, 0.0), 1
                )),
                None
            ),
            // Text box
            descriptor!(
                START_TITLE_DIALOG_TEXTBOX_GUID.0,
                WidgetDescriptor::text(text!(
                    (540.0, 90.0),
                    (60.0, 470.0),
                    28.0,
                    (255.0, 255.0, 255.0, 220.0),
                    3
                )),
                None
            ),
            descriptor!(
                START_TITLE_NAME_TEXTBOX_GUID.0,
                WidgetDescriptor::text(text!(
                    (125.0, 35.0),
                    (65.0, 395.0),
                    // (127.5, 407.5), // Center the text
                    24.0,
                    (255.0, 255.0, 255.0, 220.0),
                    3
                )),
                None
            ),
            descriptor!(
                START_TITLE_DIALOG_TEXTBOX_DUMMY_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((787.0, 142.0), (13.0, 450.0), 4)),
                None
            ),
        ]))
    )];
}

lazy_static! {
    pub static ref EXTRA_TITLE_LAYOUT: Vec<Descriptor> = vec![descriptor!(
        EXTRA_TITLE_BG_GUID.0,
        WidgetDescriptor::image(image!((WINDOW_WIDTH, WINDOW_HEIGHT), (0.0, 0.0), 0)),
        Some(GroupDescriptor(vec![
            descriptor!(
                EXTRA_TITLE_CG_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((100.0, 40.0), (200.0, 480.0), 1)),
                None
            ),
            descriptor!(
                EXTRA_TITLE_SCENE_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((100.0, 40.0), (300.0, 480.0), 1)),
                None
            ),
            descriptor!(
                EXTRA_TITLE_MUSIC_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((100.0, 40.0), (400.0, 480.0), 1)),
                None
            ),
            descriptor!(
                EXTRA_TITLE_BACK_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((100.0, 40.0), (500.0, 480.0), 1)),
                None
            )
        ]))
    ),];
}

lazy_static! {
    pub static ref CONFIG_SPEED_TITLE_LAYOUT: Vec<Descriptor> = vec![descriptor!(
        CONFIG_SPEED_TITLE_BG_GUID.0,
        WidgetDescriptor::image(image!((WINDOW_WIDTH, WINDOW_HEIGHT), (0.0, 0.0), 1)),
        Some(GroupDescriptor(vec![
            descriptor!(
                CONFIG_SPEED_TITLE_SPEED_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((130.0, 39.0), (62.0, 514.0), 1)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_SOUND_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((130.0, 39.0), (212.0, 514.0), 1)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_EXTRA_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((130.0, 39.0), (362.0, 514.0), 1)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_BACK_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((115.0, 48.0), (665.0, 528.0), 1)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_SKIP_READED_ON_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((36.0, 19.0), (372.0, 344.0), 1)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_SKIP_READED_OFF_BUTTON_GUID.0,
                WidgetDescriptor::button(button!((36.0, 24.0), (437.0, 339.0), 1)),
                None
            )
        ]))
    )];
}
