use std::collections::HashMap;
use std::fs;
use std::path::Path;
use bevy::prelude::*;
use regex::Regex;
use game_system::models::environment::{Area, Environment, EnvironmentListResource, EnvironmentState};

pub struct EnvInitPlugin;

/// The `EnvInitPlugin` is a Bevy plugin responsible for initializing the game's environments.
/// It registers a system that loads environment data during the `PreStartup` phase.
impl Plugin for EnvInitPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_environment_system);
    }
}

/// `setup_environment_system` is responsible for loading all available environments
/// from the `assets/environments` directory and storing them as a resource.
///
/// This function is executed during the `PreStartup` phase to ensure that
/// environment data is available when the game begins.
#[coverage(off)]
pub fn setup_environment_system(mut commands: Commands) {
    let environments = load_environments();
    commands.insert_resource(EnvironmentListResource(environments));
}

/// Loads all available environments from the `assets/environments` directory.
/// Each environment corresponds to a folder inside `assets/environments`,
/// and it contains multiple areas.
///
/// Returns:
/// - `HashMap<String, Environment>`: A mapping of environment names to `Environment` structs.
pub fn load_environments() -> HashMap<String, Environment> {
    let mut environments = HashMap::new();

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("assets/environments");
    
    debug!("Loading environments from {}", path.display());
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                let areas = load_areas(file_name.as_str());

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

/// Loads all areas from a given environment folder inside `assets/environments`.
/// Areas are 3D models stored as `.glb` files, following a specific naming pattern:
/// `area_<number>.glb`. The numbers determine the order of the areas.
///
/// Parameters:
/// - `folder: &str`: The name of the environment folder to scan.
///
/// Returns:
/// - `HashMap<String, Area>`: A mapping of area file names to `Area` structs.
fn load_areas(folder: &str) -> HashMap<String, Area> {
    let mut areas = HashMap::new();

    let regex = Regex::new(r"^area_(\d+)\.glb$").unwrap();

    if let Ok(contents) = fs::read_dir(format!("assets/environments/{}", folder)) {
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

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod unit_tests {
    use crate::environment::init::load_environments;

    #[test]
    fn test_load_environments_reads_directory_correctly() {

        let result = load_environments();

        assert_eq!(result.len(), 1);
    }
}