use bevy::prelude::*;
use serde::Deserialize;
use crate::entities::interact::{NearbyTarget, SensorTarget};

/// Runtime component for a **non-player character (NPC)**.
///
/// Attach this to the NPC’s **owner/root entity** (the same entity that carries AI,
/// transforms, etc.). Systems such as movement, AI, and animation read this data to
/// decide how the NPC should behave and which animations to play.
///
/// # Fields
/// - [`state`](#structfield.state): High-level NPC state used by movement/animation.
/// - [`displayed_npc`](#structfield.displayed_npc): Metadata/identifier of the visual NPC to show
///   (e.g., used to resolve animation sets or assets).
/// - [`walk_speed`](#structfield.walk_speed): Movement speed (units/sec) when walking.
/// - [`sprinting_speed`](#structfield.sprinting_speed): Movement speed (units/sec) when sprinting.
///
/// # Defaults
/// `Default::default()` initializes:
/// - `state = WorldNpcState::Idle`
/// - `displayed_npc = NpcData { id: "", name: "", locations: vec![] }`
/// - `walk_speed = 4.0`
/// - `sprinting_speed = 6.0`
///
/// The component is `Reflect' able, making it editable in inspectors and serializable if needed.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct WorldNpc {
    /// Current high-level state of the NPC (e.g., `Idle`, `Walking`, `Sprinting`).
    pub state: WorldNpcState,

    /// The data describing which NPC to display (IDs, name, spawn locations, etc.).
    /// Often used as a lookup key for assets/animations.
    pub displayed_npc: NpcData,

    /// Horizontal movement speed when the NPC is walking (units per second).
    pub walk_speed: f32,

    /// Horizontal movement speed when the NPC is sprinting (units per second).
    pub sprinting_speed: f32
}

impl Default for WorldNpc {
    /// Provides a sensible idle/default NPC configuration.
    ///
    /// - Starts in `WorldNpcState::Idle`
    /// - Uses an empty `NpcData` (no id/name/locations)
    /// - `walk_speed = 4.0`
    /// - `sprinting_speed = 6.0`
    fn default() -> Self {
        Self {
            displayed_npc: NpcData {
                id: "".to_string(),
                name: "".to_string(),
                locations: vec![]
            },
            state: WorldNpcState::Idle,
            walk_speed: 4.0,
            sprinting_speed: 6.0
        }
    }
}

/// Represents a non-player character (NPC) in the game world.
///
/// <p>This struct holds identification, name, locations where the NPC
/// can appear, and a list of dialog entries it can use.</p>
#[derive(Component, Reflect, Debug, Deserialize, Clone)]
#[reflect(Component)]
pub struct NpcData {
    /// Unique NPC identifier.
    pub id: String,

    /// Display name of the NPC.
    pub name: String,

    /// All possible positions where this NPC may appear.
    pub locations: Vec<NpcLocation>
}

/// Represents a single NPC location and its interaction trigger.
///
/// <p>Each location defines the world coordinates and an associated
/// event triggered when the player interacts with the NPC.</p>
#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct NpcLocation {
    /// X coordinate in the world.
    pub x: f32,

    /// Y coordinate in the world.
    pub y: f32,

    /// Z coordinate in the world.
    pub z: f32
}

#[derive(Component, Resource, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum WorldNpcState {
    /// The default state, representing when the player is idle and not moving.
    #[default]
    Idle,

    /// The state when the player is walking at normal speed.
    Walking,

    /// The state when the player is sprinting and moving at a faster speed.
    Sprinting
}

/// Resource that stores the currently nearby NPC (if any).
///
/// <p>Used for proximity detection and dialog triggering.</p>
#[derive(Resource, Default)]
pub struct NearbyNpc(pub Option<Entity>);

impl NearbyTarget for NearbyNpc {
    /// Sets the entity that is currently nearby.
    fn set(&mut self, value: Option<Entity>) {
        self.0 = value;
    }

    /// Returns the current nearby NPC entity.
    fn get(&self) -> Option<Entity> {
        self.0
    }
}

/// Component attached to the NPC sensor trigger volumes.
///
/// <p>Used to identify the NPC that should be activated when
/// the player enters this sensor zone.</p>
#[derive(Component)]
pub struct NpcSensor(pub Entity);

impl SensorTarget for NpcSensor {
    /// Returns the associated NPC entity.
    fn target_entity(&self) -> Entity {
        self.0
    }
}