use bevy::prelude::*;
use bevy_extended_ui::registry::UiRegistry;
use game_system::app_state::GameState;

pub struct LoadingScreen;

impl Plugin for LoadingScreen {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Preload), display_loading_screen);
    }
}

#[coverage(off)]
fn display_loading_screen(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.remove_and_use("account_screen","loading_screen");
}