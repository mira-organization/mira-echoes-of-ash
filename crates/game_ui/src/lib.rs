#![feature(coverage_attribute)]

mod screens;
mod controller;
mod inventory;
mod menu;
mod hud;

use bevy::prelude::*;
use bevy_extended_ui::ExtendedUiPlugin;
use crate::controller::UiControllerPlugin;
use crate::hud::HudPlugin;
use crate::inventory::InventoryUiPlugin;
use crate::menu::MenuPlugin;
use crate::screens::ScreenPlugin;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtendedUiPlugin);
        app.add_plugins((
            ScreenPlugin, 
            MenuPlugin,
            InventoryUiPlugin,
            HudPlugin,
            UiControllerPlugin
        ));
    }
}