use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::save_info::LoadedAssets;

pub struct LoadService;

impl Plugin for LoadService {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_assets_ready.run_if(in_state(GameState::LoadGameAssets)));
    }
}

#[coverage(off)]
fn check_assets_ready(
    asset_server: Res<AssetServer>,
    assets: Res<LoadedAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let all_loaded = assets
        .characters
        .iter()
        .all(|(_, handle)| asset_server.is_loaded(handle));
    
    if all_loaded {
        info!("Loaded all assets");
        next_state.set(GameState::InGame);
    }
}