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
use crate::models::ui::OpenUI;
use crate::save_info::{AllCharacters, AssetLoadProgress, AuthData, AuthResponse, ChangeCharacter, CurrentWorldCharacter};

pub const CHARACTER_JSON_PATH: &str = "assets/models/characters/data";
pub const CHARACTER_MODEL_PATH: &str = "models/characters/model";

pub struct GameSystemPlugin;

impl Plugin for GameSystemPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_resource::<AssetLoadProgress>();
        app.init_resource::<AuthData>();
        app.init_resource::<AuthResponse>();
        app.init_resource::<OpenUI>();
        app.insert_resource(ConfigService::new());
        app.insert_resource(CurrentWorldCharacter::default());
        app.insert_resource(ChangeCharacter(false));
        app.insert_resource(AllCharacters::default());
        app.add_plugins((ModelRegistryPlugin, EventRegistryPlugin));
    }
}