use bevy::prelude::*;
use serde::Deserialize;
use crate::entities::non_player::NpcData;

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

/// Represents the root file containing all area-based NPCs.
///
/// <p>This is the deserialized representation of the NPC configuration
/// file, containing all defined NPCs grouped by area.</p>
#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct NpcFile {
    /// A list of area-specific NPC entries.
    pub areas: Vec<AreaNpcList>,
}