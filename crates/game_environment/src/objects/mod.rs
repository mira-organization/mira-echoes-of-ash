mod item_world_placer;

use bevy::prelude::*;
use crate::objects::item_world_placer::ItemWorldPlacer;

pub struct GameObjectsPlugin;

impl Plugin for GameObjectsPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(ItemWorldPlacer);
    }
}