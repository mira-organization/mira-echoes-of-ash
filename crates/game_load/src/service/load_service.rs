use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::save_info::{AssetLoadProgress, ChangeCharacter, LoadedAssets};

pub struct LoadService;

impl Plugin for LoadService {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
         app.add_systems(Update, check_assets_ready
                             .run_if(in_state(GameState::LoadGameAssets)
                                 .or(in_state(GameState::PreloadEnv).and(resource_changed::<LoadedAssets>))));
    }
}

#[coverage(off)]
fn check_assets_ready(
    asset_server: Res<AssetServer>,
    assets: Res<LoadedAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut change_character: ResMut<ChangeCharacter>,
    mut progress: ResMut<AssetLoadProgress>,
) {
    let mut all_handles: Vec<UntypedHandle> = Vec::new();
    for handle in assets.characters.values() {
        all_handles.push(handle.clone().untyped());
    }

    for handle in assets.environments.iter() {
        all_handles.push(UntypedHandle::Weak(handle.clone()));
    }

    let total = all_handles.len();
    let mut loaded = 0;
    let mut pending = Vec::new();

    for handle in &all_handles {
        if asset_server.is_loaded_with_dependencies(handle) {
            loaded += 1;
        } else {
            pending.push(handle.clone());
        }
    }
    
    *progress = AssetLoadProgress {
        total,
        loaded,
        untyped_pending: pending,
    };
    
    if loaded == total {
        info!("All assets loaded.");
        change_character.0 = true;
        next_state.set(GameState::InGame);
    }
}