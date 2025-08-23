#![coverage(off)]

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a single item in the inventory system.
///
/// Fields:
/// - `name`: Name of the item.
/// - `description`: A textual description of the item.
/// - `icon`: Optional path to the item's icon image.
/// - `rarity`: String representation of the item's rarity (e.g., "Common", "Epic").
/// - `value`: Numeric value of the item (e.g., for selling or comparing worth).
/// - `type_`: Type or category of the item (e.g., "Weapon", "Artifact").
#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[reflect(Component)]
pub struct Item {
    pub name: String,
    pub display: String,
    pub description: String,
    pub icon: Option<String>,
    pub rarity: String,
    #[serde(default)]
    pub value: u32,

    /// The type of the item (e.g., "Weapon", "Artifact", etc.).
    #[serde(rename = "type")]
    pub type_: String,
}

/// Represents an item that exists in the game world with a physical position.
///
/// This component can be attached to an entity in the world to indicate it is a collectable item.
#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[reflect(Component)]
pub struct WorldItem {
    /// The item data (e.g., name, icon, rarity, etc.)
    pub item: Item,

    /// The spatial location of the item in the game world.
    pub location: ItemLocation,
}

/// Represents a fixed 3D position of an item in the world.
///
/// Used by [`WorldItem`] to store where the item is located.
#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ItemLocation {
    /// World position on the X-axis
    pub x: f32,

    /// World position on the Y-axis
    pub y: f32,

    /// World position on the Z-axis
    pub z: f32,
}

/// Represents a collection of items used in loot tables or inventories.
///
/// This struct is typically used for defining lists of available items in a game context.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ItemTable {
    /// List of items in the table.
    pub entries: Vec<Item>,
}