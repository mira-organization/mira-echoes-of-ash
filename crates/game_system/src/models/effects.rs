use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Debug, Clone, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Effects {
    pub name: String,
    pub duration: f64,
}