mod movement;
mod interact;

use bevy::prelude::*;
use crate::input::interact::InteractPlugin;
use crate::input::movement::MovementPlugin;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementPlugin, InteractPlugin));
    }
}