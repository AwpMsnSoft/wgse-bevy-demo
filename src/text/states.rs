use bevy::prelude::*;
use crate::text::stepped_iterator::MonoChars;

#[derive(Debug, Clone)]
pub(crate) enum TextRenderingState<'a> {
    Pendding(String),
    Rendering(MonoChars<'a>),
    Done,
}

