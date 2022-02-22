use crate::{
    button, descriptor, image,
    ui::{
        descriptors::{ButtonDescriptor, Descriptor, ImageDescriptor, WidgetDescriptor},
        resources::{
            BACK_BUTTON_GUID, CG_BUTTON_GUID, CONFIG_BUTTON_GUID, EXIT_BUTTON_GUID,
            EXTRA_BUTTON_GUID, EXTRA_TITLE_BG_GUID, MAIN_TITLE_BG_GUID, MAIN_TITLE_RES_MAP,
            MUSIC_BUTTON_GUID, SCENE_BUTTON_GUID, START_BUTTON_GUID, EXTRA_TITLE_RES_MAP,
        },
    },
};
use bevy::prelude::*;
use lazy_static::lazy_static;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

lazy_static! {
    pub static ref MAIN_TITLE_LAYOUT: Vec<Descriptor> = vec![
        descriptor!(
            MAIN_TITLE_BG_GUID.0,
            WidgetDescriptor::image(image!(
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                0.0,
                WINDOW_HEIGHT,
                String::from(*MAIN_TITLE_RES_MAP.get(&MAIN_TITLE_BG_GUID).unwrap())
            ))
        ),
        descriptor!(
            START_BUTTON_GUID.0,
            WidgetDescriptor::button(button!(100.0, 40.0, 200.0, WINDOW_HEIGHT - 480.0))
        ),
        descriptor!(
            CONFIG_BUTTON_GUID.0,
            WidgetDescriptor::button(button!(100.0, 40.0, 300.0, WINDOW_HEIGHT - 480.0))
        ),
        descriptor!(
            EXTRA_BUTTON_GUID.0,
            WidgetDescriptor::button(button!(100.0, 40.0, 400.0, WINDOW_HEIGHT - 480.0))
        ),
        descriptor!(
            EXIT_BUTTON_GUID.0,
            WidgetDescriptor::button(button!(100.0, 40.0, 500.0, WINDOW_HEIGHT - 480.0))
        ),
    ];
}

lazy_static! {
    pub static ref EXTRA_TITLE_LAYOUT: Vec<Descriptor> = vec![
        descriptor!(
            EXTRA_TITLE_BG_GUID.0,
            WidgetDescriptor::image(image!(
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                0.0,
                WINDOW_HEIGHT,
                String::from(*EXTRA_TITLE_RES_MAP.get(&EXTRA_TITLE_BG_GUID).unwrap())
            ))
        ),
        descriptor!(
            CG_BUTTON_GUID.0,
            WidgetDescriptor::button(button!(100.0, 40.0, 200.0, WINDOW_HEIGHT - 480.0))
        ),
        descriptor!(
            SCENE_BUTTON_GUID.0,
            WidgetDescriptor::button(button!(100.0, 40.0, 300.0, WINDOW_HEIGHT - 480.0))
        ),
        descriptor!(
            MUSIC_BUTTON_GUID.0,
            WidgetDescriptor::button(button!(100.0, 40.0, 400.0, WINDOW_HEIGHT - 480.0))
        ),
        descriptor!(
            BACK_BUTTON_GUID.0,
            WidgetDescriptor::button(button!(100.0, 40.0, 500.0, WINDOW_HEIGHT - 480.0))
        ),
    ];
}
