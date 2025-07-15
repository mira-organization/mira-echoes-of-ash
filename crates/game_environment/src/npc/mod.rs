mod npc_world_placer;

use bevy::prelude::*;
use crate::npc::npc_world_placer::NpcWorldPlacer;

pub struct GameNpcPlugin;

impl Plugin for GameNpcPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(NpcWorldPlacer);
    }
}