#![feature(coverage_attribute)]

pub mod characters;
pub mod models;
pub mod save_info;
mod service;

use bevy::prelude::*;
use crate::models::ModelRegistryPlugin;
use crate::service::ServicePlugin;

pub struct GameSystemPlugin;

impl Plugin for GameSystemPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((ModelRegistryPlugin, ServicePlugin));
    }
}