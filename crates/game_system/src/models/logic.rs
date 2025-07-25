use std::fs;
use std::path::Path;
use bevy::prelude::*;
use serde::Deserialize;
use crate::ENTITY_JSON_PATH;
use crate::characters::Character;

/// Marks the primary camera entity in the game.
#[derive(Component)]
pub struct MainCamera;

/// Represents a character loaded from a JSON file.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct JSONEntity {
    pub localized: String,
    /// Character's first name.
    pub name: String,
    /// Character model file name.
    pub model: String,
    /// Attack range in the world.
    pub world_attack_range: f32,
    /// List of animations associated with the character.
    pub animations: Vec<EntityAnimation>,
    /// type for internal usage
    #[serde(default, rename = "type")]
    pub _type: EntityDataType
}

#[coverage(off)]
impl JSONEntity {
    /// Loads a character from a JSON file based on their localized.
    ///
    /// # Arguments
    /// - `localized_name` - The localized of the character.
    ///
    /// # Returns
    /// - `Ok(JSONCharacter)` if successfully loaded.
    /// - `Err(String)` if the file cannot be read or parsed.
    #[coverage(off)]
    pub fn fetch(localized_name: &str) -> Result<Self, String>
    where
        Self: Sized + for<'de> serde::Deserialize<'de>,
    {
        let mut parts = localized_name.split("::");
        let folder = parts.next().ok_or("No folder in localized_name")?;
        let name = parts.next().ok_or("No name in localized_name")?;

        let file_name = if name.ends_with(".json") {
            name.to_string()
        } else {
            format!("{}.json", name)
        };

        let path = Path::new(ENTITY_JSON_PATH)
            .join(folder)
            .join("data")
            .join(&file_name);

        let file_content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

        let mut entity: Self = serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse JSON in {:?}: {}", path, e))?;
        
        entity._type = match folder {
            "characters" => EntityDataType::Character,
            "enemies" => EntityDataType::Enemy,
            "npcs" => EntityDataType::Npc,
            _ => EntityDataType::Unknown,
        };

        Ok(entity)
    }

    #[coverage(off)]
    pub fn fetch_all() -> Result<Vec<Self>, String>
    where
        Self: Sized + for<'de> serde::Deserialize<'de>,
    {
        let mut result = Vec::new();

        let dirs = fs::read_dir(ENTITY_JSON_PATH)
            .map_err(|e| format!("Failed to read directory: {}", e))?;

        for entry in dirs {
            let entry = entry.map_err(|e| format!("Failed to read dir entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                let parent_folder = path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| format!("Failed to get folder name for {:?}", path))?;

                let data_path = path.join("data");
                if data_path.exists() && data_path.is_dir() {
                    for json_entry in fs::read_dir(&data_path)
                        .map_err(|e| format!("Failed to read data dir: {}", e))?
                    {
                        let json_entry = json_entry.map_err(|e| format!("Failed to read file: {}", e))?;
                        let json_path = json_entry.path();

                        if json_path.is_file()
                            && json_path.extension().and_then(|s| s.to_str()) == Some("json")
                        {
                            let stem = json_path.file_stem()
                                .and_then(|s| s.to_str())
                                .ok_or_else(|| format!("Failed to get file stem for {:?}", json_path))?;

                            let localized_name = format!("{}::{}", parent_folder, stem);

                            let entity = Self::fetch(&localized_name)
                                .map_err(|e| format!("Failed to fetch {}: {}", localized_name, e))?;
                            result.push(entity);
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    /// Retrieves an animation by its name.
    ///
    /// # Arguments
    /// - `name` - The key of the animation.
    ///
    /// # Returns
    /// - `Some(&CharacterAnimation)` if found.
    /// - `None` if no matching animation exists.
    #[coverage(off)]
    pub fn get_animation_by_name(&self, name: &str) -> Option<&EntityAnimation> {
        self.animations.iter().find(|anim| anim.key == name)
    }
}

/// Represents an animation associated with a character.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct EntityAnimation {
    /// The animation key (e.g., "idle", "walk").
    pub key: String,
    /// The index of the animation in the `.glb` file.
    pub index: u32,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub enum EntityDataType {
    Character,
    Npc,
    Enemy,
    #[default]
    Unknown,
}

impl EntityDataType {

    #[coverage(off)]
    pub fn path(&self) -> &'static str {
        match self {
            EntityDataType::Character => "/characters/model",
            EntityDataType::Npc => "/npcs/model",
            EntityDataType::Enemy => "/enemies/model",
            EntityDataType::Unknown => "/unknown",
        }
    }
}

/// Represents a world-level player with attributes like action points
/// and movement speeds (walking and sprinting).
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct WorldPlayer {
    /// The number of action points available to the player.
    pub actions_points: usize,
    /// The player's walking speed.
    pub walk_speed: f32,
    /// The player's sprinting speed.
    pub sprinting_speed: f32,
    /// The player's step height, which is allowed.
    pub max_step_height: f32,
    /// The in world state for handle animations.
    pub state: WorldPlayerState,
    /// The attack box for hit detection.
    //pub attack_hit_box: AttackHitBox,

    // The character behind this entity
    pub displayed_character: Character,
}

impl Default for WorldPlayer {
    /// Provides default values for a `WorldPlayer`.
    /// - `actions_points`: 3
    /// - `walk_speed`: 3.0
    /// - `sprinting_speed`: 4.5
    fn default() -> Self {
        Self {
            actions_points: 3,
            walk_speed: 4.85,
            sprinting_speed: 7.5,
            max_step_height: 1.0,
            state: WorldPlayerState::default(),
/*            attack_hit_box: AttackHitBox::default(),*/
            displayed_character: Character::default()
        }
    }
}

/// The `WorldPlayerState` enum represents the different possible states of a player in the world.
///
/// This enum is used to track and manage the state of a player, such as whether the player is idle, walking, or sprinting.
/// It is particularly useful for controlling player movement and behavior within the game world.
///
/// # Variants
/// - `Idle`: The player is not moving and is in a resting state.
/// - `Walking`: The player is walking at a normal speed.
/// - `Sprinting`: The player is moving at an increased speed (sprinting).
#[derive(Component, Resource, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum WorldPlayerState {
    /// The default state, representing when the player is idle and not moving.
    #[default]
    Idle,

    /// The state when the player is walking at normal speed.
    Walking,

    /// The state when the player is sprinting and moving at a faster speed.
    Sprinting,
}

/// Represents the state of the World Inspector UI.
///
/// This resource holds a single boolean value indicating whether the World Inspector UI
/// is currently visible or hidden. The state can be toggled by user input (e.g., a key press),
/// and this struct is used to track the visibility of the World Inspector in the application.
///
/// The `WorldInspectorState` is initialized to `false` (hidden) by default.
///
/// # Fields
///
/// * `0`: A boolean value that represents the visibility of the World Inspector UI.
///   - `true`: The World Inspector is visible.
///   - `false`: The World Inspector is hidden.
#[derive(Resource, Default, Debug)]
pub struct WorldInspectorState(pub bool);

/// Trait for sensor components that target another entity
pub trait SensorTarget {
    fn target_entity(&self) -> Entity;
}

/// Trait for resource that stores an optional nearby entity
pub trait NearbyTarget {
    fn set(&mut self, value: Option<Entity>);
    fn get(&self) -> Option<Entity>;
}

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn default_world_player_values() {
        let player = WorldPlayer::default();

        // Check default numerical values
        assert_eq!(player.actions_points, 3);
        assert!((player.walk_speed - 4.85).abs() < f32::EPSILON);
        assert!((player.sprinting_speed - 7.5).abs() < f32::EPSILON);
        assert!((player.max_step_height - 1.0).abs() < f32::EPSILON);

        // Check the default state
        assert_eq!(player.state, WorldPlayerState::Idle);

        // Check default character (assumes Character::default() has known default values)
        let default_character = Character::default();
        assert_eq!(player.displayed_character, default_character);
    }

    #[test]
    fn world_player_state_enum_behaves_correctly() {
        let idle = WorldPlayerState::Idle;
        let walking = WorldPlayerState::Walking;
        let sprinting = WorldPlayerState::Sprinting;

        assert_ne!(idle, walking);
        assert_ne!(walking, sprinting);
        assert_ne!(sprinting, idle);

        assert_eq!(WorldPlayerState::default(), WorldPlayerState::Idle);
    }
}