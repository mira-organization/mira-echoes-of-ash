pub mod effects;
pub mod party;
pub mod logic;
pub mod environment;
pub mod animation;
pub mod audio;
pub mod inventory;
pub mod ui;
pub mod npcs;

use bevy::prelude::*;
use bevy_rapier3d::prelude::Group;
use crate::characters::Character;
use crate::models::effects::Effects;
use crate::models::inventory::{GameItemList, InventoryOpen, InventoryState, Item, NearbyItem, WorldItem};
use crate::models::party::CharacterPartyInfo;

pub const GROUP_CAMERA_COLLIDER: Group = Group::GROUP_1;
pub const GROUP_ITEMS_COLLIDER: Group = Group::GROUP_2;

pub struct ModelRegistryPlugin;

impl Plugin for ModelRegistryPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.register_type::<Character>();
        app.register_type::<Effects>();
        app.register_type::<Item>();
        app.register_type::<WorldItem>();
        app.init_resource::<CharacterPartyInfo>();
        app.init_resource::<GameItemList>();
        app.init_resource::<NearbyItem>();
        app.init_resource::<InventoryOpen>();
        app.init_resource::<InventoryState>();
    }
}

