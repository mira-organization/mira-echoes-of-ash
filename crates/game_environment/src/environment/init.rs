use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use bevy::prelude::*;
use regex::Regex;
use serde::Deserialize;
use game_system::models::environment::{Area, Environment, EnvironmentListResource, EnvironmentState};
use game_system::models::inventory::{GameItemList, Item, ItemLocation, WorldItem};

#[derive(Deserialize, Debug)]
struct RawObjectives {
    items: Vec<RawObjectiveItem>,
}

#[derive(Deserialize, Debug)]
struct RawObjectiveItem {
    inventory_item: String,
    amount: u32,
    location: ItemLocation,
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
                let area_path = base_path.join(&file_name);
                let areas = load_areas(file_name.as_str());
                
                let mut world_items: Vec<WorldItem> = vec![];
                let objectives_path = area_path.join("objectives.json");
                if objectives_path.exists() {
                    match read_to_string(&objectives_path) { 
                        Ok(content) => match serde_json::from_str::<RawObjectives>(&content) { 
                            Ok(raw_objectives) => {
                                for raw_item in raw_objectives.items {
                                    if let Some(item_def) = game_item_list.0.get(&raw_item.inventory_item) {
                                        world_items.push(WorldItem {
                                            item: Item {
                                                name: item_def.name.clone(),
                                                display: item_def.display.clone(),
                                                value: raw_item.amount,
                                                icon: item_def.icon.clone(),
                                                description: item_def.description.clone(),
                                                rarity: item_def.rarity.clone(),
                                                type_: item_def.type_.clone()
                                            },
                                            location: raw_item.location.clone()
                                        });
                                    } else {
                                        warn!("Item {} not found in game item list", raw_item.inventory_item);
                                    }
                                }
                            }
                            Err(err) => {
                                error!("Failed to parse {:?}: {}", objectives_path, err);
                            }
                        },
                        Err(err) => {
                            error!("Failed to read {:?}: {}", objectives_path, err);
                        }
                    }
                }

                let mut items_map = HashMap::new();
                items_map.insert(file_name.clone(), world_items);
                let environment = Environment {
                    name: file_name.clone(),
                    loaded: false,
                    areas,
                    items: items_map,
                    state: EnvironmentState::Exploring,
                };

                environments.insert(file_name, environment);
            }
        }
    }

    environments
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
                    Some((usize::MAX, file_name))
                }
            })
            .collect();

        entries.sort_by_key(|&(num, _)| num);

        for (index, file_name) in entries {
            let area = Area {
                id_name: folder.to_string(),
                name: file_name.clone(),
                index,
                player_in_bound: false,
                battle_scenes: HashMap::new(),
            };

            areas.insert(file_name, area);
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