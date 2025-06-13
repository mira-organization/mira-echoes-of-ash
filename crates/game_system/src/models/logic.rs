use bevy::prelude::*;
use crate::characters::Character;

/// Marks the primary camera entity in the game.
#[derive(Component)]
pub struct MainCamera;

/// Represents a world-level player with attributes like action points
/// and movement speeds (walking and sprinting).
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct WorldPlayer {
    /// The number of action points available to the player.
    pub actions_points: usize,
    /// The player's walking speed.
    pub walk_speed: f32,
    /// The player's sprinting speed.
    pub sprinting_speed: f32,
    /// The player's step height, which is allowed.
    pub max_step_height: f32,
    /// The in world state for handle animations.
    pub state: WorldPlayerState,
    /// The attack box for hit detection.
    //pub attack_hit_box: AttackHitBox,

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
            sprinting_speed: 7.5,
            max_step_height: 1.0,
            state: WorldPlayerState::default(),
/*            attack_hit_box: AttackHitBox::default(),*/
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
}
