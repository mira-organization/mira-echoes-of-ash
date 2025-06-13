#![feature(coverage_attribute)]

pub mod characters;
pub mod models;
pub mod save_info;
pub mod app_state;

use bevy::prelude::*;
use crate::app_state::GameState;
use crate::models::ModelRegistryPlugin;

pub struct GameSystemPlugin;

impl Plugin for GameSystemPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins(ModelRegistryPlugin);
    }
}