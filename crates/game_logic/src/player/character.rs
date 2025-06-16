use std::collections::HashMap;
use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::bundles::world_player::WorldPlayerBundle;
use game_system::CHARACTER_MODEL_PATH;
use game_system::characters::Character;
use game_system::config::ConfigService;
use game_system::models::animation::Animations;
use game_system::models::logic::{JSONCharacter, WorldPlayer};
use game_system::models::party::CharacterPartyInfo;
use game_system::save_info::{AllCharacters, ChangeCharacter, CurrentWorldCharacter, LoadedAssets};
use game_system::utils::convert;

pub struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_switch_character.run_if(in_state(GameState::InGame)));
        app.add_systems(Update, switch_character
            .run_if(resource_changed::<ChangeCharacter>)
            .run_if(resource_exists::<LoadedAssets>));
    }
}

/// Detects character switch input and sets the `ChangeCharacter` resource accordingly.
///
/// This function listens for key presses mapped to character selection and updates
/// the `change_character` flag if a valid input is detected.
///
/// # Parameters
/// - `dummy_save_data`: Stores the currently active character.
/// - `keyboard`: Handles keyboard input.
/// - `general_config`: Stores key bindings for character selection.
/// - `change_character`: A flag that determines if a character switch should occur.
#[coverage(off)]
fn trigger_switch_character(
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
    mut change_character: ResMut<ChangeCharacter>,
    mut party: ResMut<CharacterPartyInfo>
) {
    let key_map: HashMap<usize, KeyCode> = vec![
        (1, convert(&general_config.input_config.character_01).expect("Failed to convert key 1")),
        (2, convert(&general_config.input_config.character_02).expect("Failed to convert key 2")),
        (3, convert(&general_config.input_config.character_03).expect("Failed to convert key 3")),
        (4, convert(&general_config.input_config.character_04).expect("Failed to convert key 4")),
    ].into_iter().collect();

    if change_character.0 {
        return;
    }

    for (slot, keycode) in &key_map {
        if keyboard.just_pressed(*keycode) {
            if let Some((character_key, (_s, character))) = party
                .members.clone()
                .iter_mut()
                .find(|(_, (s, _))| s == slot)
            {
                if party.active.name != character.name {
                    party.active = character.clone();
                    change_character.0 = true;
                    info!("Switched character to '{}'", character_key);
                }
            }
            
            break;
        }
    }
    
}

/// Handles the actual character switching process by updating the game state.
///
/// This function:
/// - De-spawns the current character if necessary.
/// - Loads the new character model and animations.
/// - Updates the `CharacterParty` and `CurrentWorldCharacter` resources.
/// - Spawns the new character entity into the world.
///
/// # Parameters
/// - `commands`: Used to modify the entity world (spawn/de-spawn entities).
/// - `change_character`: Tracks whether a character change should happen.
/// - `dummy_save_data`: Stores the active character data.
/// - `asset_server`: Loads assets such as character models and animations.
/// - `graphs`: Stores animation graphs for character animations.
/// - `character_party`: Manages the list of available characters.
/// - `current_world_character`: Stores the currently active world character.
#[coverage(off)]
fn switch_character(
    mut commands: Commands,
    mut change_character: ResMut<ChangeCharacter>,
    mut current_world_character: ResMut<CurrentWorldCharacter>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
    party: Res<CharacterPartyInfo>,
    assets: Res<LoadedAssets>,
    query_transform: Query<&Transform, With<Character>>,
    all_characters: Res<AllCharacters>
) {
    if change_character.0 {
        let mut graph = AnimationGraph::new();
        
        let mut json_character = None;
        for raw_character in all_characters.0.iter() {
            if raw_character.name == party.active.name {
                json_character = Some(raw_character);
                break;
            }
        }
        
        if let Some(data) = json_character {
            let mut transform = Transform::from_xyz(40.0, 13.0, 40.0);
            if let Some((entity, current_character)) = current_world_character.0.clone() {
                if current_character.name == data.name {
                    change_character.0 = false;
                    return;
                }
                
                transform = match query_transform.get(entity) { 
                    Ok(t) => t.clone(),
                    Err(_) => return,
                };
                
                commands.entity(entity).despawn();
                current_world_character.0 = None;
            }

            let mut character = None;
            for (key, (_, members)) in party.members.clone() {
                if key.eq(&data.name) {
                    character = Some(members.clone());
                    break;
                }
            }
            
            if let Some(mut character) = character {
                let mut scene = Default::default();
                for (key, handles) in assets.characters.clone() {
                    if key.eq(&character.name.clone()) {
                        scene = handles;
                    }
                }

                let animations = graph
                    .add_clips(
                        [
                            GltfAssetLabel::Animation(JSONCharacter::get_animation_by_name(&data, "idle").unwrap().index as usize)
                                .from_asset(format!("{}/{}.glb", CHARACTER_MODEL_PATH, character.model_path.clone())),
                            GltfAssetLabel::Animation(JSONCharacter::get_animation_by_name(&data, "walk").unwrap().index as usize)
                                .from_asset(format!("{}/{}.glb", CHARACTER_MODEL_PATH, character.model_path.clone())),
                            GltfAssetLabel::Animation(JSONCharacter::get_animation_by_name(&data, "sprint").unwrap().index as usize)
                                .from_asset(format!("{}/{}.glb", CHARACTER_MODEL_PATH, character.model_path.clone())),
                            GltfAssetLabel::Animation(JSONCharacter::get_animation_by_name(&data, "idle-02").unwrap().index as usize)
                                .from_asset(format!("{}/{}.glb", CHARACTER_MODEL_PATH, character.model_path.clone())),
                        ].into_iter().map(|path| asset_server.load(path)),
                        1.0, graph.root).collect();
                let graph = graphs.add(graph);
                
                commands.insert_resource(Animations {
                    animations,
                    graph: graph.clone(),
                });
                
                character.in_world = true;
                let entity = commands.spawn((
                    SceneRoot(scene.clone()),
                    WorldPlayerBundle {
                        transform,
                        world_player: WorldPlayer {
                            displayed_character: character.clone(),
                            ..default()
                        },
                        ..default()
                    },
                    character.clone()
                )).id();
                
                current_world_character.0 = Some((entity, character.clone()));
                info!("Loading character: {}", character.name);
            }
        }

        change_character.0 = false;
    }
}