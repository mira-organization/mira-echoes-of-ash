#![feature(coverage_attribute)]

pub mod characters;
pub mod models;
pub mod save_info;
pub mod app_state;
pub mod utils;
pub mod config;
pub mod bundles;
pub mod events;

use bevy::prelude::*;
use crate::app_state::GameState;
use crate::config::ConfigService;
use crate::events::EventRegistryPlugin;
use crate::models::ModelRegistryPlugin;

pub struct GameSystemPlugin;

impl Plugin for GameSystemPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.insert_resource(ConfigService::new());
        app.add_plugins((ModelRegistryPlugin, EventRegistryPlugin));
    }
}