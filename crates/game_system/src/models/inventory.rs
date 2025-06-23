use bevy::prelude::*;

#[derive(Resource, Default, Clone, Debug)]
pub struct InventoryOpen(pub bool);

#[derive(Resource, Default, Clone, Debug)]
pub struct InventoryState {
    pub tab: InventoryTab,
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