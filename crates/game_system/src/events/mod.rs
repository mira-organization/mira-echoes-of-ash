pub mod player_events;

use bevy::prelude::*;
use crate::events::player_events::PlayerEvents;

pub struct EventRegistryPlugin;

impl Plugin for EventRegistryPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerEvents);
    }
}