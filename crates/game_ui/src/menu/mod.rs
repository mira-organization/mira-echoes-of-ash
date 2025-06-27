mod menu_system;

use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use bevy_extended_ui::registry::UiRegistry;
use game_system::app_state::GameState;
use crate::menu::menu_system::MenuSystem;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuSystem);
        app.add_systems(OnEnter(GameState::SplashScreen), load_up_uis);
    }
}

#[coverage(off)]
fn load_up_uis(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.add(String::from("pause_menu_screen"), HtmlSource::from_file_path("assets/html/pause_menu_screen.html"));
}