mod inventory_system;

use bevy::prelude::*;
use crate::inventory::inventory_system::InventorySystem;

pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(InventorySystem);
    }
}