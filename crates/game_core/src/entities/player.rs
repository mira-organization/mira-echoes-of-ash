use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::prelude::*;
use crate::entities::character::Character;

/// Represents a world-level player with attributes like action points
/// and movement speeds (walking and sprinting).
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct WorldPlayer {
    /// The number of action points available to the player.
    pub actions_points: usize,
    /// The player's walking speed.
    pub walk_speed: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub vertical_velocity: f32,
    /// The player's sprinting speed.
    pub sprinting_speed: f32,
    /// The player's step height, which is allowed.
    pub max_step_height: f32,
    /// The in world state for handle animations.
    pub state: WorldPlayerState,
    /// The attack box for hit detection.
    //pub attack_hit_box: AttackHitBox,

    // The character behind this entity
    pub displayed_character: Character,
}

impl Default for WorldPlayer {
    /// Provides default values for a `WorldPlayer`.
    /// - `actions_points`: 3
    /// - `walk_speed`: 3.0
    /// - `sprinting_speed`: 4.5
    fn default() -> Self {
        Self {
            actions_points: 3,
            walk_speed: 4.85,
            jump_speed: 12.5,
            gravity: 20.0,
            vertical_velocity: 0.0,
            sprinting_speed: 7.5,
            max_step_height: 1.0,
            state: WorldPlayerState::default(),
            displayed_character: Character::default()
        }
    }
}

/// The `WorldPlayerState` enum represents the different possible states of a player in the world.
///
/// This enum is used to track and manage the state of a player, such as whether the player is idle, walking, or sprinting.
/// It is particularly useful for controlling player movement and behavior within the game world.
///
/// # Variants
/// - `Idle`: The player is not moving and is in a resting state.
/// - `Walking`: The player is walking at a normal speed.
/// - `Sprinting`: The player is moving at an increased speed (sprinting).
#[derive(Component, Resource, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum WorldPlayerState {
    /// The default state, representing when the player is idle and not moving.
    #[default]
    Idle,

    /// The state when the player is walking at normal speed.
    Walking,

    /// The state when the player is sprinting and moving at a faster speed.
    Sprinting,

    Jumping,
    
    SlowWalk
}

/// A Bevy bundle grouping all components required to spawn and control
/// a player character in the world.
///
/// This bundle includes naming, rendering, physics, and character‐controller
/// components for an animated player entity, ensuring it behaves correctly
/// within the game world.
///
/// Derived Traits:
/// - `Bundle`: Marks this struct as a collection of components that can be
///   inserted together into an entity.
#[derive(Bundle)]
pub struct WorldPlayerBundle {
    pub name: Name,
    pub no_frustum_culling: NoFrustumCulling,
    pub transform: Transform,
    pub world_player: WorldPlayer,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
    pub damping: Damping,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub kinematic_controller: KinematicCharacterController,
}

#[coverage(off)]
impl Default for WorldPlayerBundle {

    fn default() -> Self {
        Self {
            name: Name::new("Active Player"),
            no_frustum_culling: NoFrustumCulling,
            transform: Transform::from_xyz(40.0, 14.0, 40.0),
            world_player: WorldPlayer::default(),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            gravity_scale: GravityScale(3.0),
            damping: Damping {
                angular_damping: 2.0,
                linear_damping: 2.0,
            },
            locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            collider: Collider::capsule(Vec3::new(0.0, 0.2, 0.0), Vec3::new(0.0, 1.6, 0.0), 0.2),
            kinematic_controller: KinematicCharacterController {
                max_slope_climb_angle: 45_f32.to_radians(),
                min_slope_slide_angle: 35_f32.to_radians(),
                autostep: Some(CharacterAutostep {
                    include_dynamic_bodies: true,
                    min_width: CharacterLength::Absolute(0.05),
                    max_height: CharacterLength::Absolute(0.55),
                }),
                snap_to_ground: Some(CharacterLength::Absolute(0.075)),
                ..default()
            },
        }
    }
}