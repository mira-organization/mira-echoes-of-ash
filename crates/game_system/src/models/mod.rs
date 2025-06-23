pub mod effects;
pub mod party;
pub mod logic;
pub mod environment;
pub mod animation;
pub mod audio;
pub mod inventory;

use bevy::prelude::*;
use crate::characters::Character;
use crate::models::effects::Effects;
use crate::models::inventory::{InventoryOpen, InventoryState};
use crate::models::party::CharacterPartyInfo;

pub struct ModelRegistryPlugin;

impl Plugin for ModelRegistryPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.register_type::<Character>();
        app.register_type::<Effects>();
        app.init_resource::<CharacterPartyInfo>();
        app.init_resource::<InventoryOpen>();
        app.init_resource::<InventoryState>();
    }
}

