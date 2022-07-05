use bevy::prelude::*;

/// `SwitchChangedEvent` is emitted when a `Switch` widget changed it's `active_child`.
pub struct SwitchChangedEvent(pub Option<Entity>);