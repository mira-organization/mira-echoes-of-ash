use bevy::prelude::*;
use serde::Deserialize;

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