use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use bevy::prelude::*;
use regex::Regex;
use serde::Deserialize;
use game_core::entities::item::{Item, ItemLocation, WorldItem};
use game_core::global_resources::GlobalItems;
use game_core::states::AppState;
use game_core::world::environment::{Area, Environment, EnvironmentListResource, EnvironmentState};
use game_core::world::non_players::{AreaNpcList, NpcFile};

#[derive(Debug, Deserialize)]
struct ItemsFile {
    pub items: Vec<ObjectiveItem>,
}
#[derive(Debug, Deserialize)]
struct ObjectiveItem {
    pub inventory_item: String,
    pub amount: u32,
    pub location: ItemLocation,
}

pub struct WorldInitHandler;

impl Plugin for WorldInitHandler {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_environment_system
            .run_if(in_state(AppState::Preload)
                .and(resource_added::<GlobalItems>)));
    }
}

/// `setup_environment_system` is responsible for loading all available environments
/// from the `assets/environments` directory and storing them as a resource.
///
/// This function is executed during the `PreStartup` phase to ensure that
/// environment data is available when the game begins.
#[coverage(off)]
pub fn setup_environment_system(mut commands: Commands, game_item_list: Res<GlobalItems>) {
    let environments = load_environments(&game_item_list);
    commands.insert_resource(EnvironmentListResource(environments));
}

/// Loads all available environments from the `assets/environments` directory.
/// Each environment corresponds to a folder inside `assets/environments`,
/// and it contains multiple areas.
///
/// Returns:
/// - `HashMap<String, Environment>`: A mapping of environment names to `Environment` structs.
#[coverage(off)]
pub fn load_environments(game_item_list: &GlobalItems) -> HashMap<String, Environment> {
    let mut environments = HashMap::new();

    let base_path = get_assets_base_path();
    debug!("Loading environments from {}", base_path.display());

    let Ok(env_entries) = fs::read_dir(&base_path) else { return environments };

    for env_dir in env_entries.flatten() {
        let env_name = match env_dir.file_name().into_string() {
            Ok(s) => s,
            Err(_) => continue,
        };
        let env_path = env_dir.path();
        if !env_path.is_dir() { continue; }

        let mut areas = load_areas_in_env(&env_name, &env_path, game_item_list);

        info!("Loaded environment {} with {} areas", env_name, areas.len());

        let environment = Environment {
            name: env_name.clone(),
            loaded: false,
            areas: std::mem::take(&mut areas),
            state: EnvironmentState::Exploring,
        };

        environments.insert(env_name, environment);
    }

    environments
}

/// Loads objectives from a JSON file and converts them into world‐item mappings.
///
/// This function attempts to read the file at `objectives_path` and parse it as an
/// `ObjectivesFile`. For each `ObjectiveArea` found, it constructs a list of `WorldItem`
/// instances by looking up item definitions in `game_item_list`. If an objective’s
/// `inventory_item` key is not found in the global item list, a warning is emitted.
/// File read or parse errors are logged as errors. Areas are keyed by their lowercase name.
///
/// # Parameters
///
/// - `objectives_path`: Path to the objective JSON file to load.
/// - `game_item_list`: Global registry of item definitions (`GlobalItems`).
///
/// # Returns
///
/// A `HashMap<String, Vec<WorldItem>>` where each key is an area name (lowercased), and the
/// value is the vector of `WorldItem` objects successfully constructed for that area.
#[coverage(off)]
fn load_area_items(area_path: &Path, game_item_list: &GlobalItems) -> Vec<WorldItem> {
    let items_path = area_path.join("items.json");
    if !items_path.exists() {
        return vec![];
    }

    let Ok(content) = fs::read_to_string(&items_path) else {
        error!("Failed to read {:?}", items_path);
        return vec![];
    };

    let Ok(file) = serde_json::from_str::<ItemsFile>(&content) else {
        error!("Failed to parse items.json {:?}", items_path);
        return vec![];
    };

    let mut out = Vec::with_capacity(file.items.len());
    for obj in file.items {
        if let Some(def) = game_item_list.0.get(&obj.inventory_item) {
            out.push(WorldItem {
                item: Item {
                    name: def.name.clone(),
                    value: obj.amount,
                    icon: def.icon.clone(),
                    display: def.display.clone(),
                    rarity: def.rarity.clone(),
                    type_: def.type_.clone(),
                    description: def.description.clone(),
                },
                location: obj.location,
            });
        } else {
            warn!("Item {} not found in game item list", obj.inventory_item);
        }
    }
    out
}

/// Loads all areas from a given environment folder inside `assets/environments`.
/// Areas are 3D models stored as `.glb` files, following a specific naming pattern:
/// `area_<number>.glb`. The numbers determine the order of the areas.
///
/// Parameters:
/// - `folder: &str`: The name of the environment folder to scan.
///
/// Returns:
/// - `HashMap<String, Area>`: A mapping of area file names to `Area` structs.
#[coverage(off)]
fn load_areas_in_env(
    env_name: &str,
    env_path: &Path,
    game_item_list: &GlobalItems,
) -> HashMap<String, Area> {
    let mut result = HashMap::new();

    let Ok(dir) = fs::read_dir(env_path) else { return result };

    let mut area_names: Vec<String> = dir
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            if p.is_dir() && p.join("terrain.glb").exists() {
                e.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect();

    natural_sort_in_place(&mut area_names);

    for (index, area_folder) in area_names.into_iter().enumerate() {
        let area_path = env_path.join(&area_folder);

        // Basis-Area
        let mut area = Area {
            id_name: env_name.to_string(),
            name: area_folder.to_lowercase(),
            index,
            items: HashMap::new(),
            non_player_characters: HashMap::new()
        };

        let world_items = load_area_items(&area_path, game_item_list);
        if !world_items.is_empty() {
            area.items.insert(area.name.clone(), world_items);
        }

        result.insert(area.name.clone(), area);
    }

    result
}

/// Loads non-player character data from a JSON file into area‐based mappings.
///
/// This function reads the file at `npc_path` and parses it as an `NpcFile`. Each
/// `AreaNpcList` is inserted into the result map using the area’s lowercase name as
/// the key. File read or parse errors are logged as errors.
///
/// # Parameters
///
/// - `npc_path`: Path to the NPC JSON file to load.
///
/// # Returns
///
/// A `HashMap<String, AreaNpcList>` mapping each area name (lowercased) to its
/// corresponding `AreaNpcList` from the file.
#[coverage(off)]
fn load_non_players(npc_path: &Path) -> HashMap<String, AreaNpcList> {
    let mut npc_map = HashMap::new();

    if npc_path.exists() {
        if let Ok(content) = read_to_string(npc_path) {
            match serde_json::from_str::<NpcFile>(&content) {
                Ok(npc_file) => {
                    for area in npc_file.areas {
                        npc_map.insert(area.name.to_lowercase(), area);
                    }
                }
                Err(err) => {
                    error!("Failed to parse NPCs {:?}: {}", npc_path, err);
                }
            }
        } else {
            error!("Failed to read {:?}", npc_path);
        }
    }

    npc_map
}

/// Returns the absolute base path to the `assets/environments` directory depending on the execution context.
/// <p>
/// When running in a development environment (e.g., from an IDE or using `cargo run`),
/// this function will detect if the executable resides in `target/debug` or `target/release`.
/// In that case, it traverses up to the project root and appends `assets/environments`.
/// <p>
/// When running from a packaged release (where the executable is located next to the `assets` directory),
/// it simply appends `assets/environments` directly next to the executable.
/// <p>
/// This ensures that the correct assets path is resolved in both development and release environments.
///
/// @return A [`PathBuf`] representing the resolved base path to the environments assets folder.
#[coverage(off)]
pub fn get_assets_base_path() -> PathBuf {
    let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new("."));

    if exe_dir.ends_with("debug") || exe_dir.ends_with("release") {
        exe_dir
            .parent()
            .and_then(|p| p.parent())
            .unwrap_or_else(|| Path::new("."))
            .join("assets/map")
    } else {
        exe_dir.join("assets/map")
    }
}

fn natural_sort_in_place(names: &mut [String]) {
    let re = Regex::new(r"(\\d+)").unwrap();
    names.sort_by(|a, b| {
        let anum = re.captures(a).and_then(|c| c.get(1)).and_then(|m| m.as_str().parse::<usize>().ok());
        let bnum = re.captures(b).and_then(|c| c.get(1)).and_then(|m| m.as_str().parse::<usize>().ok());

        match (anum, bnum) {
            (Some(na), Some(nb)) => na.cmp(&nb).then_with(|| a.cmp(b)),
            (Some(_), None)      => Ordering::Less,
            (None, Some(_))      => Ordering::Greater,
            (None, None)         => a.cmp(b),
        }
    });
}