use std::collections::{HashMap, HashSet};
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use bevy_extended_ui::registry::UiRegistry;
use game_core::entities::character::CharacterPartyInfo;
use game_core::entities::item::{Item, ItemTable};
use game_core::ENTITY_MODEL_PATH;
use game_core::global_resources::{GlobalEntities, GlobalItems};
use game_core::json::entity::{EntityDataType, JsonEntity};
use game_core::loading::LoadedAssets;
use game_core::network::save_resource::SaveData;
use game_core::states::{AppState, AssetLoadState, BeforeUiState, FetchState};
use game_core::world::environment::{Area, CurrentEnvironment, Environment, EnvironmentListResource};

pub struct PreLoadService;

impl Plugin for PreLoadService {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Preload), (
            load_item_tables,
            load_json_entities,
            pre_load_extended_ui
        ));

        app.add_systems(OnEnter(AppState::NetworkFetch(FetchState::FetchingComplete)),
                        pre_load_environments.run_if(resource_exists::<SaveData>),
        );

        app.add_systems(Update, fetch_character_from_web_backend
            .run_if(in_state(AppState::NetworkFetch(FetchState::FetchingComplete))
            .and(resource_added::<SaveData>)
                .and(resource_exists::<CurrentEnvironment>)));
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
                             save_data: Res<SaveData>,
                             mut next_game_state: ResMut<NextState<AppState>>,
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

    next_game_state.set(AppState::AssetsLoad(AssetLoadState::EnvPreLoad));
}

/// Loads all JSON-defined entities into the global game state.
///
/// This system function attempts to fetch all entities defined in JSON format
/// via `JsonEntity::fetch_all()`. On success, it replaces the contents of
/// the `GlobalEntities` resource with the fetched list. On failure, it logs
/// an error and returns early without modifying the resource.
///
/// After loading, this function also logs a debug summary of how many entities
/// of each type were loaded.
///
/// # Parameters
///
/// - `global_entities`: A mutable reference to the `GlobalEntities` resource,
///   which will be populated with the fetched entities on success.
///
/// # Behavior
///
/// 1. Calls `JsonEntity::fetch_all()`.
/// 2. If fetching fails, logs an error with the failure reason and returns.
/// 3. If fetching succeeds:
///    - Updates `global_entities.0` with the newly fetched entities.
///    - Logs a debug message with the total count of loaded entities.
///    - Iterates over all loaded entities and categorizes them by their
///      `EntityDataType`, counting how many are `Character`, `Npc`, `Enemy`,
///      or `Unknown`.
///    - Logs a debug message with the per-type counts.
#[coverage(off)]
fn load_json_entities(
    mut global_entities: ResMut<GlobalEntities>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let entities = match JsonEntity::fetch_all() {
        Ok(entities) => entities,
        Err(err) => {
            error!("Failed to load json entities: {}", err);
            return;
        }
    };

    global_entities.0 = entities;
    debug!("Loaded {} entities", global_entities.0.len());

    let mut characters = 0;
    let mut npc = 0;
    let mut enemies = 0;
    let mut unknown = 0;

    for entry in global_entities.0.iter() {
        match entry._type {
            EntityDataType::Character => characters = characters + 1,
            EntityDataType::Enemy => enemies = enemies + 1,
            EntityDataType::Npc => npc = npc + 1,
            EntityDataType::Unknown => unknown = unknown + 1,
        }
    }

    debug!("Loaded asset [ {} characters, {} npc, {} enemies, {} unknown ]", characters, npc, enemies, unknown);
    next_state.set(AppState::Screen(BeforeUiState::Splash));
}

#[coverage(off)]
fn load_item_tables(mut commands: Commands) {
    let mut game_item_list: HashMap<String, Item> = HashMap::new();
    debug!("Loading item tables...");
    let dir_path = Path::new("assets/item_tables");

    let entries = match read_dir(dir_path) {
        Ok(entries) => entries,
        Err(err) => {
            error!("Failed to read item tables directory: {}", err);
            return;
        }
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let file_content = match read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                error!("Failed to read item table file: {}", err);
                continue;
            }
        };

        let item_tables: ItemTable = match serde_json::from_str(&file_content) {
            Ok(item) => item,
            Err(err) => {
                error!("Failed to parse item table file: {}", err);
                continue;
            }
        };

        for item in item_tables.entries {
            game_item_list.insert(item.name.clone(), item);
        }
    }

    commands.insert_resource(GlobalItems(game_item_list.clone()));
    debug!("Finished loading item tables. [ {} ]", game_item_list.len());
}

/// Loads' player saves data and initializes character assets and animations based on web backend response.
///
/// This system is intended to run after the save data has been fetched from the REST backend.
/// It parses the character party information, loads the corresponding `.glb` assets and their animations,
/// and stores the data into the [`LoadedAssets`] and [`CharacterPartyInfo`] resources.
///
/// Character models and animations are loaded via the [`AssetServer`] using `GltfAssetLabel`.
/// For each party member present in the save data, the system:
/// - Loads the 3D model.
/// - Creates an [`AnimationGraph`] with standard animations (`idle`, `walk`, `sprint`, `idle-02`).
/// - Merges save data with static character info from [`AllCharacters`].
/// - Sets the active party member if applicable.
///
/// # Parameters
/// - `commands`: Used to insert the `LoadedAssets` and `SaveInfo` resources.
/// - `asset_server`: Responsible for loading GLTF assets.
/// - `graphs`: Mutable access to animation graph assets.
/// - `party`: Mutable party information (will be populated based on the save).
/// - `all_characters`: Reference data for all available characters in the game.
/// - `rest_save_data`: The save data fetched from the backend, containing the player's current party.
///
/// # Panics
/// This function panics if any of the standard animations (`idle`, `walk`, `sprint`, `idle-02`)
/// are not found in the [`JSONCharacter`] definition.
///
/// # Resources inserted
/// - [`SaveInfo`]
/// - [`LoadedAssets`]
///
/// [`LoadedAssets`]: crate::assets::LoadedAssets
/// [`CharacterPartyInfo`]: crate::character::CharacterPartyInfo
/// [`AllCharacters`]: crate::character::AllCharacters
/// [`JSONCharacter`]: crate::character::JSONCharacter
/// [`AnimationGraph`]: bevy_hierarchy_animation::graph::AnimationGraph
/// [`GltfAssetLabel`]: bevy_gltf_components::GltfAssetLabel
#[coverage(off)]
fn fetch_character_from_web_backend(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut party: ResMut<CharacterPartyInfo>,
    global_entities: Res<GlobalEntities>,
    rest_save_data: Res<SaveData>,
    current_environment: Res<CurrentEnvironment>,
) {
    let save = rest_save_data.clone();
    debug!("Loaded save for user: {}", save.username);

    let (mut entities, mut animations_map) =
        load_party_characters(&asset_server, &mut graphs, &mut party, &global_entities, &save);

    load_npc_characters(&asset_server, &mut graphs, &mut entities, &mut animations_map, &global_entities, &current_environment);

    info!("Party members loaded: {}", save.party.len());
    info!("NPCs prepared: {}", current_environment.area.non_player_characters.len());

    commands.insert_resource(save);
    commands.insert_resource(LoadedAssets {
        entities,
        environments: vec![],
        animations: animations_map,
    });
}

/// Loads all party characters defined in the player's safe data and initializes their models and animations.
///
/// This function iterates over all known characters and matches them against the party members
/// defined in the current save file. For each party member found:
/// - Loads their 3D model (GLB file) as a Bevy [`Scene`] handle.
/// - Creates an [`AnimationGraph`] and adds multiple animation clips (idle, walk, sprint, idle-02).
/// - Merges character data into the party member and registers it in the [`CharacterPartyInfo`] resource.
/// - If the member is flagged as `in_world`, sets them as the active party character.
///
/// The function returns two maps:
/// 1. `characters`: Mapping of character names to their loaded [`Scene`] handles.
/// 2. `animations_map`: Mapping of character names to their [`AnimationGraph`] handle and animation node indices.
///
/// # Parameters
/// - `asset_server`: The Bevy asset server used to load GLB models and animation clips.
/// - `graphs`: Mutable reference to the asset storage for animation graphs.
/// - `party`: Mutable reference to the player's party data resource, updated in-place.
/// - `all_characters`: Reference to all character definitions (e.g., loaded from JSON).
/// - `save`: Reference to the current player save data.
///
/// # Returns
/// A tuple containing:
/// - `HashMap<String, Handle<Scene>>`: The loaded 3D models for each party character.
/// - `HashMap<String, (Handle<AnimationGraph>, Vec<AnimationNodeIndex>)>`: The corresponding animation graphs and node indices.
///
/// # Panics
/// This function will panic if any required animation name (`idle`, `walk`, `sprint`, `idle-02`)
/// is not found in the JSON character definition.
#[coverage(off)]
fn load_party_characters(
    asset_server: &AssetServer,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    party: &mut ResMut<CharacterPartyInfo>,
    global_entities: &GlobalEntities,
    save: &SaveData,
) -> (HashMap<String, Handle<Scene>>, HashMap<String, (Handle<AnimationGraph>, Vec<AnimationNodeIndex>)>)
{
    let mut characters = HashMap::new();
    let mut animations_map = HashMap::new();
    let mut warned_animations = HashSet::new();

    for entry in &global_entities.0 {
        for member in &save.party {
            if member.name == entry.name {
                let model_path = format!("{}/{}/{}.glb", ENTITY_MODEL_PATH, entry._type.path(), entry.model_path);

                characters.insert(
                    entry.name.clone(),
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path.clone()))
                );

                let anim_names = ["idle", "idle-02", "walk", "sprint", "slow_walk", "jump"];
                let anim_handles = get_animation_handles_for_entry(
                    asset_server,
                    entry,
                    &anim_names,
                    &model_path,
                    &mut warned_animations,
                );

                let mut graph = AnimationGraph::new();
                let animations = graph.add_clips(
                    anim_handles.into_iter(),
                    1.0,
                    graph.root,
                ).collect();

                let graph = graphs.add(graph);
                let mut party_member = member.clone();
                party_member.merge_json_character(entry);
                party.add(entry.name.clone(), party_member.clone());

                if party_member.in_world {
                    party.active = party_member.clone();
                }

                animations_map.insert(entry.name.clone(), (graph, animations));
            }
        }
    }

    (characters, animations_map)
}

/// Loads non-player characters (NPCs) defined in the current environment into memory.
///
/// This function checks each NPC listed in the environment and verifies whether
/// their 3D model is already loaded (for example, if it was already loaded as a party character).
/// If the model is not yet loaded, it loads the NPCs GLB model, creates an `AnimationGraph`,
/// and registers them in the `characters` and `animations_map` collections.
///
/// Animations are added to an [`AnimationGraph`], and each NPC is associated
/// with two basic animation clips: `idle` and `walk`. The resulting animation graph
/// and animation nodes are stored in the map for later use.
///
/// # Parameters
/// - `asset_server`: The Bevy asset server used to load GLB models and animation clips.
/// - `graphs`: Mutable reference to Bevy's animation graph asset storage.
/// - `characters`: Map containing character names mapped to their loaded 3D scene handles.
/// - `animations_map`: Map containing character names mapped to their animation graph handles and node indices.
/// - `all_characters`: Reference to all available character definitions (e.g., JSON data).
/// - `current_environment`: The current game environment containing a list of non-player characters.
#[coverage(off)]
fn load_npc_characters(
    asset_server: &AssetServer,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    characters: &mut HashMap<String, Handle<Scene>>,
    animations_map: &mut HashMap<String, (Handle<AnimationGraph>, Vec<AnimationNodeIndex>)>,
    global_entities: &GlobalEntities,
    current_environment: &CurrentEnvironment,
) {
    let mut warned_animations = HashSet::new();

    for (_, npc_data) in current_environment.area.non_player_characters.iter() {
        if characters.contains_key(&npc_data.name) {
            info!("NPC '{}' already loaded as character, skipping.", npc_data.name);
            continue;
        }

        if let Some(entry) = global_entities.0.iter().find(|c| c.name == npc_data.name) {
            let model_path = format!("{}/{}/{}.glb", ENTITY_MODEL_PATH, entry._type.path(), entry.model_path);

            characters.insert(
                entry.name.clone(),
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path.clone()))
            );

            let anim_names = ["idle", "walk"];
            let anim_handles = get_animation_handles_for_entry(
                asset_server,
                entry,
                &anim_names,
                &model_path,
                &mut warned_animations,
            );

            let mut graph = AnimationGraph::new();
            let animations = graph.add_clips(
                anim_handles.into_iter(),
                1.0,
                graph.root,
            ).collect();

            let graph = graphs.add(graph);
            animations_map.insert(entry.name.clone(), (graph, animations));

            info!("Prepared NPC: {}", npc_data.name);
        } else {
            warn!("NPC data has no matching JSON character: {}", npc_data.name);
        }
    }
}

/// Gathers and loads animation clip handles for a given entity entry.
///
/// Iterates over the provided `animation_names`, looks up each one in the
/// `JsonEntity`, and issues an asset load request via the `AssetServer`.
/// For each found animation, a `Handle<AnimationClip>` is returned. If an
/// animation name is not found, a warning is emitted exactly once per
/// (entity, animation) pair, tracked by `warned_animations`.
///
/// # Parameters
///
/// - `asset_server`: Reference to Bevy’s asset server for loading GLTF sub-assets.
/// - `entry`: JSON metadata describing the entity, including animation indices.
/// - `animation_names`: Slice of animation names to load for this entity.
/// - `entity_model_path`: File path to the entity’s GLB model asset.
/// - `warned_animations`: Mutable set used to ensure each missing animation
///   warning is issued only once per entity/animation combination.
///
/// # Returns
///
/// A vector of `Handle<AnimationClip>` corresponding to the successfully
/// requested animations.
/// 'Animation0' idle, 'Animation1': crouch walk, 'Animation2' jump, 'Animation3' slow walk, 'Animation4' idle-02,
/// 'Animation5' walk, 'Animation6' sprint, 'Animation7' sprint dash, 'Animation8' T, 'Animation9' into crouch,
/// 'Animation10' crouch idle, 'Animation11' same as 10
#[coverage(off)]
fn get_animation_handles_for_entry(
    asset_server: &AssetServer,
    entry: &JsonEntity,
    animation_names: &[&str],
    entity_model_path: &str,
    warned_animations: &mut HashSet<(String, String)>,
) -> Vec<Handle<AnimationClip>> {
    let mut handles = Vec::new();

    for &anim in animation_names {
        match JsonEntity::get_animation_by_name(entry, anim) {
            Some(anim_info) => {
                let handle = asset_server.load(
                    GltfAssetLabel::Animation(anim_info.index as usize)
                        .from_asset(entity_model_path.to_string())
                );
                handles.push(handle);
            }
            None => {
                let key = (entry.name.clone(), anim.to_string());
                if warned_animations.insert(key.clone()) {
                    warn!("Animation '{}' not found for character '{}'", anim, entry.name);
                }
            }
        }
    }

    handles
}

/// Preloads extended UI definitions into the application’s UI registry.
///
/// Registers additional HTML-based UI screens or components by key, so that
/// they can be instantiated later by the UI system without a filesystem lookup
/// at runtime.
///
/// # Parameters
///
/// - `ui_registry`: Mutable reference to the global `UiRegistry` resource
///   where new UI sources are registered.
#[coverage(off)]
fn pre_load_extended_ui(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.add(String::from("account_screen"), HtmlSource::from_file_path("assets/ui/html/account.html"));
    ui_registry.add(String::from("loading_screen"), HtmlSource::from_file_path("assets/ui/html/loading_screen.html"));
    ui_registry.add(String::from("hud"), HtmlSource::from_file_path("assets/ui/html/hud.html"));
}