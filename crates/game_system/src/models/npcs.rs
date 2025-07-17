#![coverage(off)]

use bevy::prelude::*;
use serde::Deserialize;
use crate::models::logic::{NearbyTarget, SensorTarget};

#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct NpcFile {
    pub areas: Vec<AreaNpcList>,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct NpcLocation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub event: String,
    #[serde(default)]
    pub event_value: String,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct DialogAction {
    #[serde(rename = "_type")]
    pub action_type: String,
    pub id: String,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct Dialog {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub belongs_to: Option<String>,
    #[serde(rename = "_type")]
    pub dialog_type: String,
    #[serde(default)]
    pub items: Vec<String>,
    #[serde(default)]
    pub action: Option<DialogAction>,
    #[serde(default)]
    pub swap_to: Option<String>,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct AreaNpcList {
    pub name: String,
    pub list: Vec<NpcData>,
}

#[derive(Reflect, Debug, Deserialize, Clone)]
pub struct NpcData {
    pub id: String,
    pub name: String,
    pub locations: Vec<NpcLocation>,
    pub dialogs: Vec<Dialog>,
}

#[derive(Resource, Default)]
pub struct NearbyNpc(pub Option<Entity>);


impl NearbyTarget for NearbyNpc {
    fn set(&mut self, value: Option<Entity>) {
        self.0 = value;
    }

    fn get(&self) -> Option<Entity> {
        self.0
    }
}

#[derive(Component)]
pub struct NpcSensor(pub Entity);

impl SensorTarget for NpcSensor {
    fn target_entity(&self) -> Entity {
        self.0
    }
}


