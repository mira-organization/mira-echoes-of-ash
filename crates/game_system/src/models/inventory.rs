use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Clone, Debug)]
pub struct InventoryOpen {
    pub open: bool,
    pub updated: bool,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct InventoryState {
    pub tab: InventoryTab,
    pub selected_item: Option<Item>,
    pub displayed_items: Vec<Item>,
}

#[derive(Clone, Debug, Default)]
pub enum InventoryTab {
    #[default]
    All,
    Artifacts(ArtifactTab),
    Weapons(WeaponsTab),
    Accessories,
    Objects,
    Quest
}


#[derive(Clone, Debug, Default)]
pub enum ArtifactTab {
    #[default]
    All,
    Head,
    Hands,
    Chest,
    Legs,
    Boots
}

#[derive(Clone, Debug, Default)]
pub enum WeaponsTab {
    #[default]
    All,
    Swords,
    Axes,
    Bows,
    Polearms
}

#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[reflect(Component)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub rarity: String,
    pub value: u32,
    #[serde(rename = "type")]
    pub type_: String,
}