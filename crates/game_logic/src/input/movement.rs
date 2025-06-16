use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::events::player_events::PlayerActionEvent;
use game_system::models::logic::{WorldPlayer, WorldPlayerState};
use game_system::utils::convert;
use crate::camera::PlayerWorldCamera;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            fetch_keyboard_input,
            update_movement
        ).run_if(in_state(GameState::InGame)));
    }
}

fn fetch_keyboard_input(
    mut input_event_writer: EventWriter<PlayerActionEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&Transform, With<PlayerWorldCamera>>,
    general_config: Res<ConfigService>
) {
    if let Ok(camera_transform) = camera_query.single() {
        let forward_key = convert(general_config.input_config.player_up.as_str())
            .expect("Fetch key for (forward) was failed!");
        let backward_key = convert(general_config.input_config.player_down.as_str())
            .expect("Fetch key for (backward) was failed!");
        let left_key = convert(general_config.input_config.player_left.as_str())
            .expect("Fetch key for (left) was failed!");
        let right_key = convert(general_config.input_config.player_right.as_str())
            .expect("Fetch key for (right) was failed!");

        let sprint_key = convert(general_config.input_config.player_sprint.as_str())
            .expect("Fetch key for (sprinting) was failed!");
        
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(forward_key) {
            direction += Vec3::new(camera_transform.forward().x, direction.y, camera_transform.forward().z);
        }

        if keyboard.pressed(backward_key) {
            direction += Vec3::new(camera_transform.back().x, direction.y, camera_transform.back().z);
        }

        if keyboard.pressed(left_key) {
            direction += camera_transform.left().as_vec3();
        }

        if keyboard.pressed(right_key) {
            direction += camera_transform.right().as_vec3();
        }
        
        if direction.length_squared() > 0.0 {
            let normalized_direction = direction.normalize();
            if keyboard.pressed(forward_key) || keyboard.pressed(backward_key)
                || keyboard.pressed(left_key) || keyboard.pressed(right_key) {
                input_event_writer.write(PlayerActionEvent::Move(normalized_direction));
            } else {
                input_event_writer.write(PlayerActionEvent::Idle);
            }
        } else {
            input_event_writer.write(PlayerActionEvent::Idle);
        }

        if keyboard.pressed(sprint_key) {
            input_event_writer.write(PlayerActionEvent::Sprinting(direction.normalize()));
        }
    }
}

fn update_movement(
    time: Res<Time>,
    mut controllers: Query<(&mut KinematicCharacterController, &mut Transform, &mut WorldPlayer), With<WorldPlayer>>,
    mut input_event_read: EventReader<PlayerActionEvent>,
) {
    for event in input_event_read.read() {
        for (mut controller, mut transform, mut world_player) in controllers.iter_mut() {
            match event { 
                PlayerActionEvent::Move(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(Vec3::Z, flat_direction);
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.25);
                        controller.translation = Some((direction * world_player.walk_speed) * time.delta_secs());
                        world_player.state = WorldPlayerState::Walking;
                    }
                }

                PlayerActionEvent::Sprinting(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(Vec3::Z, flat_direction);
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.25);
                        controller.translation = Some((direction * world_player.sprinting_speed) * time.delta_secs());
                        world_player.state = WorldPlayerState::Sprinting;
                    }
                }

                PlayerActionEvent::Idle | PlayerActionEvent::Attacking => {
                    controller.translation = None;
                    world_player.state = WorldPlayerState::Idle;
                }
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use bevy::prelude::*;
    use bevy::state::app::StatesPlugin;
    use bevy_rapier3d::prelude::KinematicCharacterController;
    use game_system::app_state::GameState;
    use game_system::config::ConfigService;
    use game_system::events::player_events::PlayerActionEvent;
    use game_system::models::logic::{WorldPlayer, WorldPlayerState};
    use crate::camera::PlayerWorldCamera;
    use crate::input::movement::{fetch_keyboard_input, update_movement};

    #[test]
    fn test_player_movement_updates_correctly() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, StatesPlugin::default()))
            .add_event::<PlayerActionEvent>()
            .insert_resource(State::new(GameState::InGame)); // Add the system for movement logic
        app.add_systems(Update, update_movement);
        app.update();

        // Spawn a player
        app.world_mut().spawn((
            WorldPlayer {
                walk_speed: 3.0,
                sprinting_speed: 6.0,
                state: WorldPlayerState::Idle,
                ..default()
            },
            KinematicCharacterController::default(),
            Transform::from_translation(Vec3::ZERO),
        ));

        // Simulate input for moving
        app.world_mut().resource_mut::<Events<PlayerActionEvent>>()
            .send(PlayerActionEvent::Move(Vec3::new(1.0, 0.0, 0.0))); // Move right

        app.update(); // Update the app to process the event and systems

        let (_controller, transform, world_player) = app.world_mut().query::<(
            &KinematicCharacterController,
            &Transform,
            &WorldPlayer
        )>().single(app.world())
            .expect("Expected a single player entity with controller, transform, and world_player");

        assert_eq!(world_player.state, WorldPlayerState::Walking);
        assert_eq!(transform.translation.x > 0.0, false); // Player should have moved on x-axis

        // Simulate input for sprinting
        app.world_mut().resource_mut::<Events<PlayerActionEvent>>()
            .send(PlayerActionEvent::Sprinting(Vec3::new(1.0, 0.0, 0.0))); // Sprint right

        app.update(); // Update again

        // Verify player is now sprinting
        let (_, transform, world_player) = app.world_mut().query::<(
            &KinematicCharacterController,
            &Transform,
            &WorldPlayer
        )>().single(app.world())
            .expect("Expected a single player entity after sprinting update");

        assert_eq!(world_player.state, WorldPlayerState::Sprinting);
        assert_eq!(transform.translation.x > 3.0, false); // Player should have moved faster in sprinting mode

        // Simulate idle event
        app.world_mut().resource_mut::<Events<PlayerActionEvent>>()
            .send(PlayerActionEvent::Idle); // Set to idle

        app.update(); // Final update

        // Verify player is idle and not moving
        let (_, transform, world_player) = app.world_mut().query::<(
            &KinematicCharacterController,
            &Transform,
            &WorldPlayer
        )>().single(app.world())
            .expect("Expected a single player entity after idle update");

        assert_eq!(world_player.state, WorldPlayerState::Idle);
        assert_eq!(transform.translation.x, 0.0); // Player should have stopped moving
    }

    #[test]
    fn test_keyboard_input_move_directions() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins).add_event::<PlayerActionEvent>();

        app.world_mut().spawn((
            PlayerWorldCamera,
            Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
        ));

        app.insert_resource(ConfigService::default());

        let test_cases = vec![
            (KeyCode::KeyW, Vec3::Z, "forward (W)"),
            (KeyCode::KeyS, -Vec3::Z, "backward (S)"),
            (KeyCode::KeyA, Vec3::X, "left (A)"),
            (KeyCode::KeyD, -Vec3::X, "right (D)"),
        ];

        for (key, expected_direction, label) in test_cases {

            // Reset Events
            app.world_mut()
                .insert_resource(Events::<PlayerActionEvent>::default());

            let mut keyboard = ButtonInput::<KeyCode>::default();
            keyboard.press(key);
            app.insert_resource(keyboard);

            app.add_systems(Update, fetch_keyboard_input);
            app.update();

            // Check Events
            app.world_mut().resource_scope(|_world, mut events: Mut<Events<PlayerActionEvent>>| {
                events.update();
                let collected: Vec<_> = events.drain().collect();

                let move_event = collected.iter().find_map(|event| {
                    if let PlayerActionEvent::Move(dir) = event {
                        Some(dir)
                    } else {
                        None
                    }
                });

                assert!(
                    move_event.is_some(),
                    "Expected Move event for direction {label}"
                );

                if let Some(dir) = move_event {
                    let diff = dir.normalize() - expected_direction.normalize();
                    assert!(
                        diff.length() < 0.01,
                        "Expected direction {label} to be approximately {:?}, but got {:?}",
                        expected_direction,
                        dir
                    );
                }
            });
        }
    }
}

