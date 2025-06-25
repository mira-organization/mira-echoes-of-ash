use std::collections::HashMap;
use bevy::prelude::*;
use serde::Deserialize;
use crate::models::inventory::WorldItem;

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
    pub items: HashMap<String, Vec<WorldItem>>,
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
    pub battle_scenes: HashMap<String, BattleScene>
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

/// A marker component for environment-related scenes.
///
/// This component is used to tag entities representing environment visuals in the game world.
#[derive(Component, Debug, Clone)]
pub struct EnvironmentScene;

#[derive(Component, Debug, Clone)]
pub struct BattleEnvironment;

/// Stores the currently loaded area scenes as handles.
///
/// This resource maps scene layer names to their corresponding `Handle<Scene>` objects.
/// The layers include collision, environment visuals, and objects.
///
/// # Layers
/// - `"first_layer"`: Contains collision data.
/// - `"second_layer"`: Contains the visual environment.
/// - `"last_layer"`: Contains objects in the scene.
#[derive(Resource, Debug, Clone)]
pub struct CurrentAreaScenes(pub HashMap<String, Handle<Scene>>);

/// Stores a handle to a GLTF asset that is being loaded for an area.
/// Used to track the loading state of area assets.
#[derive(Resource, Debug, Clone)]
pub struct WaitingForAreaAssets(pub Handle<Gltf>);

/// Stores a handle to a GLTF asset that contains effect scene data.
/// Used for loading additional scene effects like lights or particle effects.
#[derive(Resource, Debug, Clone)]
pub struct EffectSceneAssets(pub Handle<Gltf>);

/// Represents light data extracted from GLTF extras.
/// The data is deserialized from JSON and contains parameters to configure different types of lights.
#[derive(Deserialize, Debug)]
pub struct LightData {
    /// Name of the light type (e.g., "point", "spot").
    pub name: String,
    /// Optional intensity of the light, defaults if not provided.
    pub intensity: Option<f32>,
    /// Optional range of the light, defining how far it affects its surroundings.
    pub range: Option<f32>,
    /// Optional Radius is used by Point lights only!
    pub radius: Option<f32>,
    /// RGB color values of the light, represented as an array of three floating-point numbers.
    pub color: [f32; 3],
    /// Option bool value for a handle displaying character and object shadows
    pub shadows: Option<bool>,
    /// Optional inner cone angle for spotlights, defining the sharply lit area.
    pub inner_cone: Option<f32>,
    /// Optional outer cone angle for spotlights, defining the full spread of light.
    pub outer_cone: Option<f32>
}

/// Enum representing different types of lights that can be spawned in the game.
/// - `Point` for omnidirectional point lights.
/// - `Spot` for directional spotlights with a cone shape.
pub enum LightType {
    Point(PointLight),
    Spot(SpotLight),
}