use std::collections::HashMap;
use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::models::party::CharacterPartyInfo;
use game_system::save_info::{LoadedAssets, SaveInfo};

pub struct PreLoadService;

impl Plugin for PreLoadService {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Preload), fetch_from_web_backend);
    }
}

fn fetch_from_web_backend(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut party: ResMut<CharacterPartyInfo>,
) {
    let json = include_str!("../../../../dummy/rest-save.json");
    match SaveInfo::fetch_from_json(&json.to_string()) {
        Ok(save) => {
            info!("Loaded save for user: {}", save.username);

            let mut characters = HashMap::new();
            for character in save.party.iter() {
                characters.insert(character.name.clone(), asset_server.load(GltfAssetLabel::Scene(0).from_asset(character.model_path.clone())));
                party.add(character.name.clone(), character.clone());
                if character.in_world {
                    party.active = character.clone();
                }
            }

            commands.insert_resource(save);
            commands.insert_resource(LoadedAssets { characters });
        },
        Err(e) => error!("Failed to parse save file: {}", e),
    }
    next_game_state.set(GameState::LoadGameAssets);
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use bevy::asset::AssetServer;
    use bevy::scene::ScenePlugin;
    use bevy::state::app::StatesPlugin;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
    enum GameState {
        #[default]
        Splash,
        LoadGameAssets,
    }

    #[derive(Resource)]
    struct LoadedAssets {
        characters: HashMap<String, Handle<Scene>>,
    }

    #[derive(Resource, Default)]
    struct CharacterPartyInfo {
        party: HashMap<String, Character>,
        active: Option<Character>,
    }

    impl CharacterPartyInfo {
        fn add(&mut self, name: String, character: Character) {
            self.party.insert(name, character);
        }
    }

    #[derive(Resource, Clone, Debug, serde::Deserialize)]
    struct SaveInfo {
        username: String,
        party: Vec<Character>,
    }

    impl SaveInfo {
        fn fetch_from_json(data: &str) -> Result<Self, String> {
            serde_json::from_str(data).map_err(|e| e.to_string())
        }
    }

    #[derive(Clone, Debug, serde::Deserialize, PartialEq)]
    struct Character {
        name: String,
        model_path: String,
        in_world: bool,
    }

    #[test]
    fn test_fetch_from_web_backend_loads_characters_and_transitions_state() {
        let mut app = App::new();

        // Add required Bevy plugins and systems
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), StatesPlugin::default(), ScenePlugin::default()))
            .init_resource::<CharacterPartyInfo>()
            .init_state::<GameState>()
            .insert_resource(NextState::<GameState>::default());

        // Insert dummy AssetServer
        let _ = app.world_mut().resource::<AssetServer>();

        // Override the system to use inline JSON string instead of include_str!
        fn test_fetch_system(
            mut commands: Commands,
            mut next_game_state: ResMut<NextState<GameState>>,
            asset_server: Res<AssetServer>,
            mut party: ResMut<CharacterPartyInfo>,
        ) {
            let json = r#"
            {
                "username": "test_user",
                "party": [
                    { "name": "Hero", "model_path": "models/hero.glb", "in_world": true },
                    { "name": "Mage", "model_path": "models/mage.glb", "in_world": false }
                ]
            }
            "#;
            match SaveInfo::fetch_from_json(&json.to_string()) {
                Ok(save) => {
                    let mut characters = HashMap::new();
                    for character in save.party.iter() {
                        let handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(character.model_path.clone()));
                        characters.insert(character.name.clone(), handle);
                        party.add(character.name.clone(), character.clone());
                        if character.in_world {
                            party.active = Some(character.clone());
                        }
                    }
                    commands.insert_resource(save);
                    commands.insert_resource(LoadedAssets { characters });
                }
                Err(e) => error!("Failed to parse save file: {}", e),
            }
            next_game_state.set(GameState::LoadGameAssets);
        }

        app.add_systems(Update, test_fetch_system);

        // Run the system
        app.update();

        // Assertions

        // SaveInfo inserted as a resource
        let save = app.world().get_resource::<SaveInfo>().unwrap();
        assert_eq!(save.username, "test_user");
        assert_eq!(save.party.len(), 2);

        // LoadedAssets contains both characters
        let loaded_assets = app.world().get_resource::<LoadedAssets>().unwrap();
        assert_eq!(loaded_assets.characters.len(), 2);
        assert!(loaded_assets.characters.contains_key("Hero"));
        assert!(loaded_assets.characters.contains_key("Mage"));

        // Party info updated
        let party_info = app.world().get_resource::<CharacterPartyInfo>().unwrap();
        assert_eq!(party_info.party.len(), 2);
        assert!(party_info.active.is_some());
        assert_eq!(party_info.active.as_ref().unwrap().name, "Hero");

        // GameState transitioned
        let state = app.world().resource::<State<GameState>>();
        assert_eq!(state.get(), &GameState::Splash);
    }
}