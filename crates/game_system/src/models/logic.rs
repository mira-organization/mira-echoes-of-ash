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

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn default_world_player_values() {
        let player = WorldPlayer::default();

        // Check default numerical values
        assert_eq!(player.actions_points, 3);
        assert!((player.walk_speed - 4.85).abs() < f32::EPSILON);
        assert!((player.sprinting_speed - 7.5).abs() < f32::EPSILON);
        assert!((player.max_step_height - 1.0).abs() < f32::EPSILON);

        // Check the default state
        assert_eq!(player.state, WorldPlayerState::Idle);

        // Check default character (assumes Character::default() has known default values)
        let default_character = Character::default();
        assert_eq!(player.displayed_character, default_character);
    }

    #[test]
    fn world_player_state_enum_behaves_correctly() {
        let idle = WorldPlayerState::Idle;
        let walking = WorldPlayerState::Walking;
        let sprinting = WorldPlayerState::Sprinting;

        assert_ne!(idle, walking);
        assert_ne!(walking, sprinting);
        assert_ne!(sprinting, idle);

        assert_eq!(WorldPlayerState::default(), WorldPlayerState::Idle);
    }
}