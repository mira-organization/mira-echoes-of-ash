use bevy::prelude::*;
use bevy_extended_ui::registry::UiRegistry;
use game_core::states::{AppState, AssetLoadState};

pub struct LoadingScreen;

impl Plugin for LoadingScreen {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::AssetsLoad(AssetLoadState::EnvPreLoad)), display_loading_screen);
    }
}

/// Switches the active UI from the account screen to the loading screen.
///
/// This system function uses the `UiRegistry` resource to remove the
/// `"account_screen"` entry and immediately activate the `"loading_screen"`
/// entry for display.
///
/// # Parameters
///
/// - `ui_registry`: A mutable reference to the global `UiRegistry` resource,
///   which manages registration and activation of named UI screens.
#[coverage(off)]
fn display_loading_screen(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.remove_and_use("account_screen","loading_screen");
}