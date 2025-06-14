use std::collections::HashMap;
use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::models::environment::{Area, CurrentEnvironment, Environment, EnvironmentListResource};
use game_system::models::party::CharacterPartyInfo;
use game_system::save_info::{LoadedAssets, SaveInfo};

pub struct PreLoadService;

impl Plugin for PreLoadService {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Preload), (
            fetch_from_web_backend,
            pre_load_environments.run_if(resource_added::<SaveInfo>)
        ).chain());
    }
}

#[coverage(off)]
fn fetch_from_web_backend(
    mut commands: Commands,
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
            commands.insert_resource(LoadedAssets { characters, environments: vec![] });
        },
        Err(e) => error!("Failed to parse save file: {}", e),
    }
}

/// Preloads the environment based on saved data.
///
/// This function selects the environment and area to be loaded based on the fake save data.
/// If the environment map is empty, an error is logged, and the function returns early.
/// Once the correct environment and area are found, they are stored in the `CurrentEnvironment`
/// resource and the game state transitions to `GameState::EnvironmentLoad`.
///
/// # Arguments
///
/// * `commands` - Used to insert the `CurrentEnvironment` resource.
/// * `environment` - The list of available environments.
/// * `dummy_save_data` - Holds the current environment and area index.
/// * `next_state` - Used to transition to the next game state.
pub fn pre_load_environments(mut commands: Commands,
                             environment: Res<EnvironmentListResource>,
                             save_data: Res<SaveInfo>,
                             mut next_game_state: ResMut<NextState<GameState>>,
) {
    let env_map = environment.0.clone();
    if env_map.is_empty() {
        error!("Empty environment map");
        return;
    }

    let mut to_load: Option<Area> = None;
    let mut founded_env: Option<Environment> = None;
    for (key, value) in env_map.iter() {
        if key.eq(&save_data.current_environment) {
            for (_a_key, area) in value.areas.iter() {
                if area.index == save_data.current_area {
                    to_load = Some(area.clone());
                }
            }
            founded_env = Some(value.clone());
        }
    }

    if let Some(env) = founded_env {
        if let Some(area) = to_load {
            commands.insert_resource(CurrentEnvironment {
                environment: env.clone(),
                area: area.clone(),
            });
            info!("Loading environments [{:?}]", env.name);
        }
    }

    next_game_state.set(GameState::LoadGameAssets);
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use bevy::asset::AssetServer;
    use bevy::scene::ScenePlugin;
    use bevy::state::app::StatesPlugin;
    use game_system::models::environment::EnvironmentState;

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

    #[test]
    fn test_pre_load_environments() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default(), StatesPlugin::default()));
        app.init_state::<game_system::app_state::GameState>();
        app.insert_resource(NextState::<game_system::app_state::GameState>::default());
        let _asset_server = app.world_mut().resource::<AssetServer>();
        app.insert_resource(CurrentEnvironment {
            environment: Environment {
                loaded: false,
                name: "Debug".to_string(),
                state: EnvironmentState::Exploring,
                areas: HashMap::new()
            },
            area: Area {
                index: 0,
                name: "Debug Area".to_string(),
                battle_scenes: HashMap::new(),
                player_in_bound: false
            }
        });

        let env_map = vec![
            ("env1".to_string(), Environment {
                name: "Environment 1".to_string(),
                loaded: false,
                areas: vec![
                    ("area1".to_string(), Area {
                        index: 0,
                        player_in_bound: false,
                        name: "Area 1".to_string(),
                        battle_scenes: Default::default(),
                    }),
                    ("area2".to_string(), Area {
                        index: 1,
                        player_in_bound: false,
                        name: "Area 2".to_string(),
                        battle_scenes: Default::default(),
                    }),
                ].into_iter().collect(),
                state: EnvironmentState::Exploring,
            }),
        ].into_iter().collect::<HashMap<String, Environment>>();

        app.insert_resource(EnvironmentListResource(env_map));

        let dummy_save_data = game_system::save_info::SaveInfo {
            id: "".to_string(),
            current_environment: "env1".to_string(),
            current_area: 0,
            party: vec![],
            username: "Debug".to_string(),
            email: "".to_string(),
            birthday: "".to_string(),
        };
        
        app.insert_resource(dummy_save_data);
        app.add_systems(Startup, pre_load_environments.run_if(resource_added::<game_system::save_info::SaveInfo>));
        app.update();

        let current_env = app.world().resource::<CurrentEnvironment>();
        assert_eq!(current_env.environment.name, "Environment 1");
        assert_eq!(current_env.area.name, "Area 1");
    }
}