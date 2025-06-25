use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default)]
pub struct NearbyItem(pub Option<Entity>);

#[derive(Component)]
pub struct ItemSensor(pub Entity);

#[derive(Resource, Default, Clone, Debug)]
pub struct GameItemList(pub HashMap<String, Item>);

/// Resource that indicates whether the inventory UI is currently open
/// and whether it was recently updated.
///
/// - `open`: Set to `true` when the inventory UI is visible.
/// - `updated`: Set to `true` when the inventory state has changed and should be refreshed.
#[derive(Resource, Default, Clone, Debug)]
pub struct InventoryOpen {
    pub open: bool,
    pub updated: bool,
}

/// Resource that holds the current state of the inventory UI.
///
/// - `tab`: The currently selected inventory tab.
/// - `selected_item`: The currently selected item, if any.
/// - `displayed_items`: The list of items currently displayed in the UI, filtered by the selected tab.
#[derive(Resource, Default, Clone, Debug)]
pub struct InventoryState {
    pub tab: InventoryTab,
    pub selected_item: Option<Item>,
    pub displayed_items: Vec<Item>,
}

/// Represents the main category tabs of the inventory.
///
/// Tabs:
/// - `All`: Displays all items.
/// - `Artifacts`: Subcategorized by `ArtifactTab`.
/// - `Weapons`: Subcategorized by `WeaponsTab`.
/// - `Accessories`: Items such as rings, necklaces, etc.
/// - `Objects`: Generic or miscellaneous items.
/// - `Quest`: Quest-related items that may not be usable or equitable.
#[derive(Clone, Debug, Default)]
pub enum InventoryTab {
    #[default]
    All,
    Artifacts(ArtifactTab),
    Weapons(WeaponsTab),
    Accessories,
    Objects,
    Quest,
}

/// Represents subcategories of artifacts in the inventory.
///
/// Subcategories:
/// - `All`: Shows all artifact types.
/// - `Head`: Helmets or headgear.
/// - `Hands`: Gloves, gauntlets.
/// - `Chest`: Armor or torso protection.
/// - `Legs`: Pants or leg armor.
/// - `Boots`: Footwear or boots.
#[derive(Clone, Debug, Default)]
pub enum ArtifactTab {
    #[default]
    All,
    Head,
    Hands,
    Chest,
    Legs,
    Boots,
}

/// Represents subcategories of weapons in the inventory.
///
/// Subcategories:
/// - `All`: Shows all weapon types.
/// - `Swords`: One-handed or two-handed swords.
/// - `Axes`: Hatchets, battle-axes, etc.
/// - `Bows`: Ranged bows.
/// - `Polearms`: Spears, halberds, and similar weapons.
#[derive(Clone, Debug, Default)]
pub enum WeaponsTab {
    #[default]
    All,
    Swords,
    Axes,
    Bows,
    Polearms,
}

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

#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[reflect(Component)]
pub struct WorldItem {
    pub item: Item,
    pub location: ItemLocation,
}

#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ItemLocation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ItemTable {
    pub entries: Vec<Item>,
}