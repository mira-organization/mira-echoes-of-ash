use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use game_core::states::{AppState, AssetLoadState};

pub struct LightHandler;

impl Plugin for LightHandler {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::AssetsLoad(AssetLoadState::EnvPreLoad)), create_sun_light);
    }
}

/// Spawns a directional "sun" light with cascaded shadows into the world.
///
/// This system creates a new entity named `"Sunlight"` and configures it with:
/// - A `DirectionalLight` using an illuminance level suitable for an overcast day,
///   with shadows enabled.
/// - A `Transform` positioned 200 units above the origin and rotated -45° around
///   the X axis to simulate a sunlight angle.
/// - A cascade shadow configuration with four cascades, defined bounds, and a
///   specified overlap proportion to improve shadow quality across distances.
///
/// # Parameters
///
/// - `commands`: The Bevy `Commands` buffer used to spawn and configure the light entity.
#[coverage(off)]
fn create_sun_light(
    mut commands: Commands,
) {
    info!("Creating sun light");
    commands.spawn((
        Name::new("Sun Light"),
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
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