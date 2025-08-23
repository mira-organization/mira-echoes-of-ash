use bevy::prelude::*;
use serde::Deserialize;

/// Represents light data extracted from GLTF extras.
/// The data is deserialized from JSON and contains parameters to configure different types of lights.
#[derive(Deserialize, Debug)]
pub struct LightData {
    /// Name of the light type (e.g., "point", "spot").
    pub name: String,
    /// Optional intensity of the light, defaults if not provided.
    pub intensity: Option<f32>,
    /// Optional range of the light, defining how far it affects its surroundings.
    pub range: Option<f32>,
    /// Optional Radius is used by Point lights only!
    pub radius: Option<f32>,
    /// RGB color values of the light, represented as an array of three floating-point numbers.
    pub color: [f32; 3],
    /// Option bool value for a handle displaying character and object shadows
    pub shadows: Option<bool>,
    /// Optional inner cone angle for spotlights, defining the sharply lit area.
    pub inner_cone: Option<f32>,
    /// Optional outer cone angle for spotlights, defining the full spread of light.
    pub outer_cone: Option<f32>
}

/// Enum representing different types of lights that can be spawned in the game.
/// - `Point` for omnidirectional point lights.
/// - `Spot` for directional spotlights with a cone shape.
pub enum LightType {
    Point(PointLight),
    Spot(SpotLight),
}