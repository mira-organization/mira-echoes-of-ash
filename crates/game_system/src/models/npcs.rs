#![coverage(off)]

use bevy::prelude::*;
use serde::Deserialize;
use crate::models::dialog::DialogData;
use crate::models::logic::{NearbyTarget, SensorTarget};

/// Represents the root file containing all area-based NPCs.
///
/// <p>This is the deserialized representation of the NPC configuration
/// file, containing all defined NPCs grouped by area.</p>
#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct NpcFile {
    /// A list of area-specific NPC entries.
    pub areas: Vec<AreaNpcList>,
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
    pub z: f32,

    /// Event identifier triggered at this location.
    pub event: String,

    /// Optional value for the event (e.g., a parameter or target ID).
    #[serde(default)]
    pub event_value: String,
}

/// Represents an optional action triggered by a dialog.
///
/// <p>This action may be defined in a dialog node and executed when the
/// dialog is completed or a choice is made.</p>
#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct DialogAction {
    /// The type of the action (e.g., "trigger", "quest", etc.).
    #[serde(rename = "_type")]
    pub action_type: String,

    /// The identifier associated with this action.
    pub id: String,
}

/// Represents a single dialog entry belonging to an NPC.
///
/// <p>This can include the dialog text, optional ownership metadata,
/// triggered actions, and item rewards.</p>
#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct Dialog {
    /// Unique dialog identifier.
    pub id: String,

    /// The dialog text shown to the player.
    pub text: String,

    /// Optional identifier for ownership of this dialog.
    #[serde(default)]
    pub belongs_to: Option<String>,

    /// Dialog category or type.
    #[serde(rename = "_type")]
    pub dialog_type: String,

    /// Optional list of item IDs granted during this dialog.
    #[serde(default)]
    pub items: Vec<String>,

    /// Optional action that is triggered by this dialog.
    #[serde(default)]
    pub action: Option<DialogAction>,

    /// Optional dialog ID to switch to after this one.
    #[serde(default)]
    pub swap_to: Option<String>,
}


/// Represents a collection of NPCs grouped by area.
///
/// <p>This is used to organize all NPCs within specific game areas.
/// Each area has a name and a list of corresponding `NpcData` entries.</p>
#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct AreaNpcList {
    /// The name of the area.
    pub name: String,

    /// The list of NPCs within this area.
    pub list: Vec<NpcData>,
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
    pub locations: Vec<NpcLocation>,

    /// All dialog nodes associated with this NPC.
    pub dialogs: Vec<Dialog>,
}

impl DialogData for NpcData {
    fn dialog_visible_id(&self) -> &'static str {
        "npc-dialog"
    }
    fn dialog_visible_id_placeholder() -> &'static str {
        "npc-dialog"
    }
    fn title_id(&self) -> &'static str {
        "npc-title"
    }
    fn text_id(&self) -> &'static str {
        "npc-text"
    }
    fn title_text(&self) -> String {
        self.name.clone()
    }
    fn main_text(&self, interact_key: &str) -> String {
        format!("Talk with {} [ {} ]", self.name, interact_key)
    }
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

