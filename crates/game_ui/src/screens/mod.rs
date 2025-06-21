pub mod splashscreen;
mod loading_screen;
mod hud;
mod account_screen;

use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use bevy_extended_ui::registry::UiRegistry;
use game_system::app_state::GameState;
use crate::screens::account_screen::AccountScreen;
use crate::screens::hud::HudScreen;
use crate::screens::loading_screen::LoadingScreen;
use crate::screens::splashscreen::SplashScreen;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SplashScreen,
            AccountScreen,
            LoadingScreen,
            HudScreen
        ));
        app.add_systems(OnEnter(GameState::SplashScreen), load_up_uis);
    }
}

fn load_up_uis(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.add(String::from("loading_screen"), HtmlSource::from_file_path("assets/html/loading_screen.html"));
    ui_registry.add(String::from("hud"), HtmlSource::from_file_path("assets/html/hud.html"));
    ui_registry.add(String::from("account_screen"), HtmlSource::from_file_path("assets/html/account.html"));
}