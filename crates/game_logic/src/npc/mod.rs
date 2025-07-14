mod handler;

use bevy::prelude::*;
use crate::npc::handler::NpcHandler;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(NpcHandler);
    }
}