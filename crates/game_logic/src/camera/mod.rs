pub mod logic;

use bevy::prelude::*;
use crate::camera::logic::CameraLogicPlugin;

/// The plugin responsible for managing the game camera logic.
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        // Add the camera logic plugin to the app.
        app.add_plugins(CameraLogicPlugin);
    }
}

/// A component responsible for controlling camera behavior, such as sensitivity, zoom, and offset.
#[derive(Component, Debug, Copy, Clone)]
pub struct CameraController {
    pub sensitivity: Vec2,   // Camera sensitivity for both X and Y axes.
    pub lock_active: bool,    // Indicates whether the camera is locked to the player's viewpoint.
    pub zoom: Zoom,           // Holds zoom-related parameters (min, max zoom, etc.).
    pub offset: Offset,       // Offset that modifies the camera's position relative to the player.
    pub to_head: f32,
    pub target_range: f32,    // Detection range for enemies (higher means far detection).
    pub smoother: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            sensitivity: Vec2::new(0.45, 0.45), // Default sensitivity values for the camera.
            lock_active: true, // The Camera is locked to the player by default.
            zoom: Zoom::new(1.0, 6.0), // Set the default zoom range (min: 1.0, max: 6.0).
            offset: Offset::new(0.0, 0.8), // Set default offset values for camera positioning.
            to_head: 0.6,
            target_range: 7.5,
            smoother: 0.7,
        }
    }
}

/// A marker component that identifies the camera entity associated with the player.
#[derive(Component, Reflect, Debug, Clone)]
pub struct PlayerWorldCamera;

/// Struct representing the zoom parameters for the camera.
#[derive(Clone, Copy, Debug)]
pub struct Zoom {
    pub min: f32,             // Minimum zoom level.
    pub max: f32,             // Maximum zoom level.
    pub offset_swap: f32,     // A threshold for switching between different camera zoom levels.
    pub zoom_sensitivity: f32, // Sensitivity for zooming (controls zoom speed).
    pub radius: f32,          // Current zoom level (radius of the camera's distance).
    pub target_radius: f32,   // Target zoom radius that the camera will gradually move towards.
}

impl Zoom {
    /// Creates a new Zoom instance with the specified minimum and maximum zoom levels.
    pub fn new(min: f32, max: f32) -> Self {
        let initial_radius = (min + max) / 2.0; // Start at the midpoint between min and max zoom.
        Self {
            min,
            max,
            offset_swap: min + 1.25, // Set offset swap threshold slightly higher than min zoom.
            zoom_sensitivity: 2.0,   // Set default zoom sensitivity.
            radius: initial_radius,  // Set the initial zoom radius.
            target_radius: initial_radius, // Start with the target radius equal to the initial radius.
        }
    }
}

/// Struct representing the offset values for the camera's position.
#[derive(Clone, Copy, Debug)]
pub struct Offset {
    pub offset: (f32, f32), // Tuple representing the x and y offset of the camera.
}

impl Offset {
    /// Creates a new Offset instance with the specified x and y values.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            offset: (x, y),
        }
    }
}

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;
    use bevy::math::Vec2;

    #[test]
    fn test_zoom_new_initialization() {
        let zoom = Zoom::new(1.0, 5.0);
        assert_eq!(zoom.min, 1.0);
        assert_eq!(zoom.max, 5.0);
        assert_eq!(zoom.offset_swap, 2.25);
        assert_eq!(zoom.zoom_sensitivity, 2.0);
        assert_eq!(zoom.radius, 3.0);
        assert_eq!(zoom.target_radius, 3.0);
    }

    #[test]
    fn test_offset_new_initialization() {
        let offset = Offset::new(0.2, 1.4);
        assert_eq!(offset.offset.0, 0.2);
        assert_eq!(offset.offset.1, 1.4);
    }

    #[test]
    fn test_camera_controller_default_values() {
        let controller = CameraController::default();

        assert_eq!(controller.sensitivity, Vec2::new(0.45, 0.45));
        assert!(controller.lock_active);
        assert_eq!(controller.zoom.min, 1.0);
        assert_eq!(controller.zoom.max, 6.0);
        assert_eq!(controller.zoom.radius, 3.5);
        assert_eq!(controller.offset.offset, (0.0, 0.8));
        assert_eq!(controller.to_head, 0.6);
        assert_eq!(controller.target_range, 7.5);
        assert_eq!(controller.smoother, 0.7);
    }
}
