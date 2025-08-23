#![coverage(off)]

use std::collections::HashMap;
use bevy::prelude::*;
use crate::entities::item::WorldItem;
use crate::entities::non_player::NpcData;

/// Stores a list of all available environments in the game.
///
/// The key is a `String` representing the environment name, and the value is
/// an `Environment` struct containing details about the environment.
///
/// This resource is initialized as an empty `HashMap` by default.
#[derive(Resource, Debug)]
pub struct EnvironmentListResource(pub HashMap<String, Environment>);

impl Default for EnvironmentListResource {

    #[coverage(off)]
    fn default() -> Self {
        Self {
            0: HashMap::new(),
        }
    }
}

/// Stores information about the currently active environment and area.
///
/// This resource holds both the selected `Environment` and the specific `Area`
/// within it that the player is currently in.
#[derive(Resource, Debug)]
pub struct CurrentEnvironment {
    pub environment: Environment,
    pub area: Area,
}

/// Represents an environment in the game.
///
/// An environment consists of multiple ` Areas' and has a `state`
/// that determines whether it's in an exploring, battle, or boss state.
///
/// # Fields
/// - `name`: The name of the environment.
/// - `loaded`: Whether the environment is currently loaded.
/// - `areas`: A map of areas within this environment.
/// - `state`: The current state of the environment.
#[derive(Component, Reflect, Debug, Clone)]
pub struct Environment {
    pub name: String,
    pub loaded: bool,
    pub areas: HashMap<String, Area>,
    pub state: EnvironmentState
}

/// Represents a specific area within an environment.
///
/// Each area has an index, a name, and may contain battle scenes.
/// The `player_in_bound` field indicates if the player is currently in this area.
///
/// # Fields
/// - `name`: The name of the area.
/// - `index`: The index of the area within the environment.
/// - `player_in_bound`: Whether the player is currently inside the area's boundaries.
/// - `battle_scenes`: A collection of battle scenes associated with this area.
#[derive(Reflect, Debug, Clone)]
pub struct Area {
    pub id_name: String,
    pub name: String,
    pub index: usize,
    pub player_in_bound: bool,
    pub battle_scenes: HashMap<String, BattleScene>,
    pub items: HashMap<String, Vec<WorldItem>>,
    pub non_player_characters: HashMap<String, NpcData>
}

/// Defines the possible states of an environment.
///
/// # Variants
/// - `Exploring`: The player is freely exploring the environment.
/// - `Battle`: The player is currently in a battle.
/// - `Boss`: The player is engaged in a boss fight.
#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum EnvironmentState {
    Exploring,
    Battle,
    Boss
}

/// Represents a battle scene in an area.
///
/// Each battle scene has a name and a set of associated battle music tracks.
///
/// # Fields
/// - `name`: The name of the battle scene.
/// - `battle_music`: A map of music tracks for the battle.
#[derive(Component, Reflect, Debug, Clone)]
pub struct BattleScene {
    pub name: String,
    pub battle_music: HashMap<String, String>,
}

///
/// This resource maps scene layer names to their corresponding `Handle<Scene>` objects.
/// The layers include collision, environment visuals, and objects.
///
/// # Layers
/// - `"first_layer"`: Contains collision data.
/// - `"second_layer"`: Contains the visual environment.
/// - `"Last_layer"`: Contains objects in the scene.
#[derive(Resource, Debug, Clone)]
pub struct CurrentAreaScenes(pub HashMap<String, Handle<Scene>>);

/// A marker component for environment-related scenes.
///
/// This component is used to tag entities representing environment visuals in the game world.
#[derive(Component, Debug, Clone)]
pub struct EnvironmentScene;
