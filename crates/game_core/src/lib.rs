#![feature(coverage_attribute)]

pub mod config;
pub mod states;
pub mod json;
pub mod global_resources;
pub mod network;
pub mod entities;
pub mod loading;
pub mod world;
pub mod key_converter;
pub mod collider_groups;
pub mod ui;
pub mod events;

use bevy::prelude::*;
use crate::config::ConfigModule;
use crate::entities::EntitiesModule;
use crate::events::EventRegistryPlugin;
use crate::global_resources::GlobalEntities;
use crate::loading::AssetLoadProgress;
use crate::network::NetworkModule;
use crate::ui::UiModule;
use crate::world::WorldModule;

pub const ENTITY_JSON_PATH: &str = "assets/entities/";
pub const ENTITY_MODEL_PATH: &str = "entities";

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLoadProgress>();
        app.init_resource::<GlobalEntities>();
        app.add_plugins((ConfigModule, NetworkModule, EntitiesModule, EventRegistryPlugin, WorldModule, UiModule));
    }
}

/// Represents the state of the World Inspector UI.
///
/// This resource holds a single boolean value indicating whether the World Inspector UI
/// is currently visible or hidden. The state can be toggled by user input (e.g., a key press),
/// and this struct is used to track the visibility of the World Inspector in the application.
///
/// The `WorldInspectorState` is initialized to `false` (hidden) by default.
///
/// # Fields
///
/// * `0`: A boolean value that represents the visibility of the World Inspector UI.
///   - `true`: The World Inspector is visible.
///   - `false`: The World Inspector is hidden.
#[derive(Resource, Default, Debug)]
pub struct WorldInspectorState(pub bool);