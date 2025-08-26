mod world_service;
mod movement_service;

use bevy::prelude::*;
use crate::entities::non_player::movement_service::NpcMovementService;
use crate::entities::non_player::world_service::NpcWorldService;

pub struct NonPlayerService;

impl Plugin for NonPlayerService {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((NpcWorldService, NpcMovementService));
    }
}