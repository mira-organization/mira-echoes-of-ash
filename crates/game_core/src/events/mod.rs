pub mod player_events;
pub mod non_player_events;

use bevy::prelude::*;
use crate::events::non_player_events::NonPlayerEvents;
use crate::events::player_events::PlayerEvents;

pub struct EventRegistryPlugin;

impl Plugin for EventRegistryPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerEvents, NonPlayerEvents));
    }
}