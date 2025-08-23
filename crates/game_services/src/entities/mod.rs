mod animations;
mod non_player;

use bevy::prelude::*;
use crate::entities::animations::AnimationsLogic;
use crate::entities::non_player::NonPlayerService;

pub struct EntitiesService;

impl Plugin for EntitiesService {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(NonPlayerService);
        app.add_plugins(AnimationsLogic);
    }
}