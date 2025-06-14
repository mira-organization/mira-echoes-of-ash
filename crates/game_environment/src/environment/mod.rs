pub mod init;
mod ready_handles;

use std::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::models::environment::EnvironmentListResource;
use crate::environment::init::EnvInitPlugin;
use crate::environment::ready_handles::ReadyUpHandles;

pub struct EnvironmentPlugin;

/// The main plugin responsible for managing environments in the game.
///
/// This plugin initializes environment-related resources, adds sub-plugins,
/// and registers the necessary systems related to environment management.
///
/// # Systems Added
/// - `create_light`: Handles the creation of lighting when an environment is loaded.
///
/// # Plugins Added
/// - `EnvInitPlugin`: Handles environment initialization.
/// - `ReadyUpHandles`: Manages loading environments and areas.
/// - `EnvSwapSystemPlugin`: Manages environment swapping.
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnvironmentListResource>();
        app.add_plugins((EnvInitPlugin, ReadyUpHandles));
        app.add_systems(OnEnter(GameState::LoadGameAssets), create_light);
    }
}

fn create_light(mut commands: Commands) {
    // Spawn the directional light entity
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,  // Set light intensity to an overcast day level
            shadows_enabled: true,  // Enable shadows for the light
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),  // Position the light above the floor
            rotation: Quat::from_rotation_x(-PI / 4.0),  // Rotate the light to cast shadows at an angle
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 4,  // Set up 4 cascades for better shadow quality
            first_cascade_far_bound: 10.0,  // Set the distance for the first shadow cascade
            minimum_distance: 0.5,  // Minimum distance for shadow rendering
            maximum_distance: 200.0,  // Maximum distance for shadow rendering
            overlap_proportion: 0.2  // Set the overlap proportion for shadow cascades
        }
            .build(),
    ));
}