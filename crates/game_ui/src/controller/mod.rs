mod loading_screen_controller;
mod inventory_controller;
mod menu_screen_controller;

use bevy::prelude::*;
use crate::controller::inventory_controller::InventoryController;
use crate::controller::loading_screen_controller::LoadingScreenController;
use crate::controller::menu_screen_controller::MenuScreenController;

pub struct UiControllerPlugin;

impl Plugin for UiControllerPlugin {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LoadingScreenController,
            InventoryController,
            MenuScreenController
        ));
    }
}