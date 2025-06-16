use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::save_info::{ChangeCharacter, LoadedAssets};

pub struct LoadService;

impl Plugin for LoadService {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_assets_ready.run_if(in_state(GameState::PostLoad)));
    }
}

#[coverage(off)]
fn check_assets_ready(
    asset_server: Res<AssetServer>,
    assets: Res<LoadedAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut change_character: ResMut<ChangeCharacter>,
) {
    // Check if all character assets are loaded
    let all_loaded_characters = assets
        .characters
        .values()
        .all(|handle| asset_server.is_loaded(handle));

    // Check if all environment assets are loaded
    let all_loaded_maps = assets
        .environments
        .iter()
        .all(|id| asset_server.is_loaded_with_dependencies(*id)); // ← NOTE: Use `is_loaded_with_id`

    if all_loaded_characters && all_loaded_maps {
        info!("Loaded all character and environment assets");
        change_character.0 = true;
        next_state.set(GameState::InGame);
    }
}