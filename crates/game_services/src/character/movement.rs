use bevy::prelude::*;
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};
use game_core::config::client_conf::GameConfig;
use game_core::entities::camera::PlayerWorldCamera;
use game_core::entities::character::SlowWalkToggle;
use game_core::entities::player::{WorldPlayer, WorldPlayerState};
use game_core::events::player_events::PlayerActionEvent;
use game_core::key_converter::convert;
use game_core::states::{AppState, InGameStates};

pub struct MovementLogic;

impl Plugin for MovementLogic {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_resource::<SlowWalkToggle>();
        app.add_systems(Update, (
            fetch_keyboard_input,
            update_movement
        ).run_if(in_state(AppState::InGame(InGameStates::Game))));
    }
}

/// Collects keyboard input and emits high-level `PlayerActionEvent`s for the current frame.
///
/// - Reads movement (WASD or user-configured) **relative to the camera** and projects it onto the XZ plane,
///   so camera pitch does not affect horizontal motion.
/// - Emits exactly one of `Move`/`Sprinting`/`Idle` per frame, plus a single-shot `Jump` when the jump key
///   transitions to pressed (`just_pressed`).
/// - Key bindings are looked up via the runtime `GameConfig`.
///
/// # Parameters
/// - `input_event_writer`: Event writer used to send `PlayerActionEvent`s.
/// - `keyboard`: Current keyboard state (`ButtonInput<KeyCode>`).
/// - `camera_query`: The player world camera transform; used to build camera-relative directions.
/// - `game_config`: Provides the input mapping strings which are converted to `KeyCode`s.
///
/// # Notes
/// - If the camera entity is missing (a query fails), this frame produces no input events.
/// - Jump is emitted independently and can coincide with a movement event in the same frame.
#[coverage(off)]
fn fetch_keyboard_input(
    mut input_event_writer: EventWriter<PlayerActionEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&Transform, With<PlayerWorldCamera>>,
    game_config: Res<GameConfig>,
    mut slow_toggle: ResMut<SlowWalkToggle>, // <-- neu
) {
    if let Ok(camera_transform) = camera_query.single() {
        let forward_key  = convert(game_config.input.move_up.as_str()).expect("forward key");
        let backward_key = convert(game_config.input.move_down.as_str()).expect("backward key");
        let left_key     = convert(game_config.input.move_left.as_str()).expect("left key");
        let right_key    = convert(game_config.input.move_right.as_str()).expect("right key");
        let sprint_key   = convert(game_config.input.sprint.as_str()).expect("sprint key");
        let jump_key     = convert(game_config.input.jump.as_str()).expect("jump key");
        let slow_key     = convert(game_config.input.toggle_slow_walk.as_str()).expect("slow key");

        // Toggle on key-down
        if keyboard.just_pressed(slow_key) {
            slow_toggle.0 = !slow_toggle.0;
        }

        // Build camera-relative direction on XZ
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(forward_key)  { let f = camera_transform.forward();  direction += Vec3::new(f.x, 0.0, f.z); }
        if keyboard.pressed(backward_key) { let b = camera_transform.back();     direction += Vec3::new(b.x, 0.0, b.z); }
        if keyboard.pressed(left_key)     { let l = camera_transform.left();     direction += Vec3::new(l.x, 0.0, l.z); }
        if keyboard.pressed(right_key)    { let r = camera_transform.right();    direction += Vec3::new(r.x, 0.0, r.z); }

        if direction.length_squared() > 0.0 {
            let dir_norm = direction.normalize();
            if keyboard.pressed(sprint_key) {
                input_event_writer.write(PlayerActionEvent::Sprinting(dir_norm));
            } else if slow_toggle.0 {
                input_event_writer.write(PlayerActionEvent::SlowWalk(dir_norm));
            } else {
                input_event_writer.write(PlayerActionEvent::Move(dir_norm));
            }
        } else {
            input_event_writer.write(PlayerActionEvent::Idle);
        }

        if keyboard.just_pressed(jump_key) {
            input_event_writer.write(PlayerActionEvent::Jump);
        }
    }
}

/// Applies character movement and vertical physics **once per frame** based on aggregated input events.
///
/// This system solves jitter/teleport issues by:
/// 1) Aggregating all `PlayerActionEvent`s received this frame into a single intent (dir/sprint/jump/idle).
/// 2) Applying horizontal translation and vertical motion (jump/gravity) exactly once, then setting
///    `controller.translation` a single time.
///
/// Behavior:
/// - Uses Rapier's `KinematicCharacterControllerOutput` to determine `grounded`.
/// - Applies a jump impulse only when grounded; integrates gravity only while airborne.
/// - Rotates the character smoothly (slerp) towards the movement direction on the XZ plane.
/// - Updates `WorldPlayer.state` to reflect Walking/Sprinting/Jumping/Idle.
///
/// # Parameters
/// - `time`: Bevy time resource for `delta_secs()`.
/// - `controllers`: Tuple of (`KinematicCharacterController`, optional `KinematicCharacterControllerOutput`,
///   `Transform`, and mutable `WorldPlayer`) for each controlled entity.
/// - `input_event_read`: Reader for `PlayerActionEvent's collected this frame.
///
/// # Important
/// - This system assumes it runs at most once per frame and **after** input collection.
/// - `controller.translation` is set **exactly once** to avoid conflicting motions within the same frame.
#[coverage(off)]
fn update_movement(
    time: Res<Time>,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
        &mut Transform,
        &mut WorldPlayer
    ), With<WorldPlayer>>,
    mut input_event_read: EventReader<PlayerActionEvent>,
    slow_toggle: Res<SlowWalkToggle>,
) {
    let mut dir = Vec3::ZERO;
    let mut has_move = false;
    let mut sprint = false;
    let mut slow = false;
    let mut jump = false;

    for e in input_event_read.read() {
        match e {
            PlayerActionEvent::Sprinting(d) => { dir = *d; has_move = true;  sprint = true;  slow = false; }
            PlayerActionEvent::SlowWalk(d)  => { dir = *d; has_move = true;  sprint = false; slow = true;  }
            PlayerActionEvent::Move(d)      => { dir = *d; has_move = true;  sprint = false; slow = false; }
            PlayerActionEvent::Jump         => { jump = true; }
            PlayerActionEvent::Idle         => { has_move = false; }
            PlayerActionEvent::Attacking    => { /* ignore for locomotion */ }
        }
    }

    if has_move && !sprint && slow_toggle.0 {
        slow = true;
    }

    for (mut controller, output_opt, mut transform, mut wp) in controllers.iter_mut() {
        let dt = time.delta_secs();
        let grounded = output_opt.map(|o| o.grounded).unwrap_or(true);

        if grounded && wp.vertical_velocity < 0.0 {
            wp.vertical_velocity = 0.0;
        }
        if jump && grounded {
            wp.vertical_velocity = wp.jump_speed;
        }
        if !grounded {
            wp.vertical_velocity -= wp.gravity * dt;
        }

        let mut dist = Vec3::ZERO;
        if has_move {
            let mut speed = if sprint { wp.sprinting_speed } else { wp.walk_speed };
            if !sprint && slow {
                speed = (speed - 3.0).max(0.0);
            }

            let flat = Vec3::new(dir.x, 0.0, dir.z);
            if flat.length_squared() > 0.0 {
                let flat_norm = flat.normalize();
                let target_rot = Quat::from_rotation_arc(Vec3::Z, flat_norm);
                transform.rotation = transform.rotation.slerp(target_rot, 0.25);
                dist += flat_norm * speed * dt;
            }
        }

        dist.y = wp.vertical_velocity * dt;
        controller.translation = Some(dist);

        wp.state = if jump || !grounded {
            WorldPlayerState::Jumping
        } else if has_move {
            if sprint { WorldPlayerState::Sprinting }
            else if slow { WorldPlayerState::SlowWalk }
            else { WorldPlayerState::Walking }
        } else {
            WorldPlayerState::Idle
        };
    }
}
