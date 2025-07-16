use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use bevy::prelude::*;
use regex::Regex;
use serde::Deserialize;
use game_system::models::environment::{Area, Environment, EnvironmentListResource, EnvironmentState};
use game_system::models::inventory::{GameItemList, Item, ItemLocation, WorldItem};
use game_system::models::npcs::{AreaNpcList, NpcFile};

#[derive(Debug, Deserialize)]
struct ObjectivesFile {
    pub areas: Vec<ObjectiveArea>,
}

#[derive(Debug, Deserialize)]
struct ObjectiveArea {
    pub name: String,
    pub items: Vec<ObjectiveItem>,
}


#[derive(Debug, Deserialize)]
struct ObjectiveItem {
    pub inventory_item: String,
    pub amount: u32,
    pub location: ItemLocation,
}

pub struct EnvInitPlugin;

/// The `EnvInitPlugin` is a Bevy plugin responsible for initializing the game's environments.
/// It registers a system that loads environment data during the `Startup` phase.
impl Plugin for EnvInitPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment_system);
    }
}

/// `setup_environment_system` is responsible for loading all available environments
/// from the `assets/environments` directory and storing them as a resource.
///
/// This function is executed during the `PreStartup` phase to ensure that
/// environment data is available when the game begins.
#[coverage(off)]
pub fn setup_environment_system(mut commands: Commands, game_item_list: Res<GameItemList>) {
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
pub fn load_environments(game_item_list: &GameItemList) -> HashMap<String, Environment> {
    let mut environments = HashMap::new();

    let base_path = get_assets_base_path();
    debug!("Loading environments from {}", base_path.display());

    if let Ok(entries) = fs::read_dir(&base_path) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                let env_path = base_path.join(&file_name);

                // Load areas (area_xxx.glb)
                let mut areas = load_areas(&file_name);

                // Load NPCs
                let npc_path = env_path.join("npcs.json");
                let npc_data = load_npcs(&npc_path);

                // Load objectives/items
                let objectives_path = env_path.join("objectives.json");
                let objective_data = load_objectives(&objectives_path, game_item_list);

                // Merge NPCs and items into each Area
                for (area_name, area) in areas.iter_mut() {
                    let area_key = area_name.trim_end_matches(".glb").to_lowercase();
                    // NPCs
                    if let Some(npc_area) = npc_data.get(&area_key.to_lowercase()) {
                        for npc in &npc_area.list {
                            area.non_player_characters.insert(npc.id.clone(), npc.clone());
                        }
                    }

                    // Items
                    if let Some(world_items) = objective_data.get(&area_key.to_lowercase()) {
                        area.items.insert(area_key.clone(), world_items.clone());
                    }
                }

                let environment = Environment {
                    name: file_name.clone(),
                    loaded: false,
                    areas,
                    state: EnvironmentState::Exploring,
                };

                environments.insert(file_name, environment);
            }
        }
    }

    environments
}

#[coverage(off)]
fn load_objectives(
    objectives_path: &Path,
    game_item_list: &GameItemList,
) -> HashMap<String, Vec<WorldItem>> {
    let mut area_items: HashMap<String, Vec<WorldItem>> = HashMap::new();

    if objectives_path.exists() {
        if let Ok(content) = read_to_string(objectives_path) {
            match serde_json::from_str::<ObjectivesFile>(&content) {
                Ok(objectives_file) => {
                    for area in objectives_file.areas {
                        let mut world_items = vec![];

                        for obj in area.items {
                            if let Some(item_def) = game_item_list.0.get(&obj.inventory_item) {
                                world_items.push(WorldItem {
                                    item: Item {
                                        name: item_def.name.clone(),
                                        value: obj.amount,
                                        icon: item_def.icon.clone(),
                                        display: item_def.display.clone(),
                                        rarity: item_def.rarity.clone(),
                                        type_: item_def.type_.clone(),
                                        description: item_def.description.clone()
                                    },
                                    location: obj.location,
                                });
                            } else {
                                warn!(
                                    "Item {} not found in game item list",
                                    obj.inventory_item
                                );
                            }
                        }

                        area_items.insert(area.name.to_lowercase(), world_items);
                    }
                }
                Err(err) => {
                    error!("Failed to parse objectives {:?}: {}", objectives_path, err);
                }
            }
        } else {
            error!("Failed to read {:?}", objectives_path);
        }
    }

    area_items
}

#[coverage(off)]
fn load_npcs(npc_path: &Path) -> HashMap<String, AreaNpcList> {
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
fn load_areas(folder: &str) -> HashMap<String, Area> {
    let mut areas = HashMap::new();

    let regex = Regex::new(r"^area_(\d+)\.glb$").unwrap();
    let path = get_assets_base_path().join(folder);

    if let Ok(contents) = fs::read_dir(path) {
        let mut entries: Vec<(usize, String)> = contents
            .flatten()
            .filter_map(|entry| {
                let file_name = entry.file_name().into_string().ok()?;

                if let Some(caps) = regex.captures(&file_name) {
                    let number: usize = caps[1].parse().ok()?;
                    Some((number, file_name))
                } else {
                    None
                }
            })
            .collect();

        entries.sort_by_key(|&(num, _)| num);

        for (index, file_name) in entries {
            let area = Area {
                id_name: folder.to_string(),
                name: file_name.trim_end_matches(".glb").to_lowercase().clone(),
                index,
                player_in_bound: false,
                battle_scenes: HashMap::new(),
                items: HashMap::new(),
                non_player_characters: HashMap::new(),
            };

            areas.insert(file_name.to_lowercase(), area);
        }
    }

    areas
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
            .join("assets/environments")
    } else {
        exe_dir.join("assets/environments")
    }
}