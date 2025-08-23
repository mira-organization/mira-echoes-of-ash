use bevy::prelude::*;

pub struct NonPlayerEvents;

impl Plugin for NonPlayerEvents {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_event::<NpcPreSpawnEvent>();
    }
}

#[derive(Event, Debug)]
pub struct NpcPreSpawnEvent;