use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::save_info::{LoadedAssets, SaveInfo};

pub struct PreLoadService;

impl Plugin for PreLoadService {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Preload), fetch_from_web_backend);
    }
}

fn fetch_from_web_backend(mut commands: Commands,  mut next_game_state: ResMut<NextState<GameState>>, asset_server: Res<AssetServer>) {
    let json = include_str!("../../../../dummy/rest-save.json");
    match SaveInfo::fetch_from_json(&json.to_string()) {
        Ok(save) => { 
            info!("Loaded save for user: {}", save.username);
            
            let mut characters = Vec::new();
            for character in save.party.iter() {
                characters.push(asset_server.load(GltfAssetLabel::Scene(0).from_asset(character.model_path.clone())));
            }
            
            commands.insert_resource(save);
            commands.insert_resource(LoadedAssets { characters });
        },
        Err(e) => error!("Failed to parse save file: {}", e),
    }
    next_game_state.set(GameState::LoadGameAssets);
}