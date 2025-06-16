mod animation;
mod character;

use bevy::prelude::*;
use crate::player::animation::PlayerAnimationPlugin;
use crate::player::character::PlayerCharacterPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerAnimationPlugin, PlayerCharacterPlugin));
    }
}