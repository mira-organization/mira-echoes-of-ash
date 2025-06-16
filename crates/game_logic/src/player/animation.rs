use std::time::Duration;
use bevy::prelude::*;
use game_system::app_state::GameState;
use game_system::characters::Character;
use game_system::models::logic::{WorldPlayer, WorldPlayerState};
use game_system::save_info::LoadedAssets;

/// A plugin responsible for managing player animations.
///
/// This plugin sets up the animation transitions for the player and updates
/// them based on the player's current state.
pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            setup_animation,
            update_animation
        ).run_if(in_state(GameState::InGame)).run_if(resource_exists::<LoadedAssets>));
    }
}

/// Sets up the player's animation system by initializing the animation graph and transitions.
///
/// This system runs once when a new `AnimationPlayer` component is added to the player entity.
///
/// # Parameters
/// - `commands`: Provides access to entity commands for adding components.
/// - `animations`: The resource containing animation data and the graph.
/// - `players`: Query to access entities with a newly added `AnimationPlayer`.
pub fn setup_animation(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    world_players: Query<(Entity, &Character), With<WorldPlayer>>,
) {
    for (entity, mut animation_player) in players.iter_mut() {
        let mut current_entity = entity;

        while let Ok(parent) = parents.get(current_entity) {
            current_entity = parent.parent();
            if let Ok((_, character)) = world_players.get(current_entity) {
                let mut animation_transitions = AnimationTransitions::new();
                if let Some((graph, animations)) = loaded_assets.animations.get(character.name.as_str()) {
                    animation_transitions.play(&mut animation_player, animations[0], Duration::ZERO).repeat();
                    commands.entity(entity).insert(AnimationGraphHandle(graph.clone())).insert(animation_transitions);
                    break;
                }
            }
        }
    }
}

/// Updates the player's animations based on the player's state.
///
/// This system ensures that the correct animation is played based on the `WorldPlayerState`.
///
/// # Parameters
/// - `players`: Query to access player entities and their states.
/// - `animations`: The resource containing animation data.
/// - `animation_players`: Query to access animation players and their transitions.
pub fn update_animation(
    time: Res<Time>,
    mut players: Query<&mut WorldPlayer>,
    loaded_assets: Res<LoadedAssets>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut timers: Local<Vec<Timer>>
) {
    if timers.len() < players.iter().len() {
        timers.resize_with(players.iter().len(), || Timer::new(Duration::from_secs(20), TimerMode::Repeating));
    }

    for (i, player) in players.iter_mut().enumerate() {
        for (mut animation_player, mut animation_transitions) in &mut animation_players {
            let timer = &mut timers[i];
            timer.tick(time.delta());

            if let Some((_, animations)) = loaded_assets.animations.get(player.displayed_character.name.as_str()) {
                match player.state {
                    WorldPlayerState::Idle => {
                        if timer.finished() {
                            let idle_animation_entries = [1, 1, 1];
                            let random_index = rand::random_range(0..idle_animation_entries.len());
                            let random_idle = animations[idle_animation_entries[random_index]];

                            animation_transitions.play(&mut animation_player, random_idle, Duration::from_millis(425));
                            timer.reset();
                        } else {
                            if !animation_player.is_playing_animation(animations[0]) {
                                for (current_index, active_animation) in animation_player.playing_animations_mut() {
                                    if !active_animation.is_finished() {
                                        if current_index.index() == 2 {
                                            return;
                                        }
                                    }
                                }
                                animation_transitions.play(&mut animation_player, animations[0], Duration::from_millis(425)).repeat();
                            }
                        }
                    }

                    WorldPlayerState::Walking => {
                        if !animation_player.is_playing_animation(animations[2]) {
                            animation_transitions.play(&mut animation_player, animations[2], Duration::from_millis(450)).repeat();
                        }
                        timer.reset();
                    }

                    WorldPlayerState::Sprinting => {
                        if !animation_player.is_playing_animation(animations[3]) {
                            animation_transitions.play(&mut animation_player, animations[3], Duration::from_millis(550)).repeat();
                        }
                        timer.reset();
                    }
                }
            }
        }
    }
}

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use bevy::asset::weak_handle;
    use bevy::prelude::*;
    use game_system::characters::Character;
    use game_system::models::logic::{WorldPlayer, WorldPlayerState};
    use game_system::save_info::LoadedAssets;
    use crate::player::animation::{setup_animation, update_animation};

    #[test]
    fn test_setup_assigns_animation_graph_and_transitions() {
        let mut app = App::new();

        // Fake Graph & Index
        let dummy_graph = weak_handle!("b76ec96e-5c93-4a4c-ae3c-26d54ef36f9e");
        let dummy_index: AnimationNodeIndex = 0.into();

        let mut animations_map = HashMap::new();
        animations_map.insert(
            "TestChar".to_string(),
            (dummy_graph.clone(), vec![dummy_index]),
        );

        app.insert_resource(LoadedAssets {
            characters: HashMap::new(),
            animations: animations_map,
            environments: vec![],
        });

        let world_player_entity = app.world_mut().spawn((
            Character {
                name: "TestChar".to_string(),
                ..default()
            },
            WorldPlayer::default(),
            Transform::default(),
            GlobalTransform::default(),
        )).id();

        let child = app.world_mut().spawn((
            Name::new("AnimatedEntity"),
            AnimationPlayer::default(),
            Transform::default(),
            GlobalTransform::default(),
        )).id();

        app.world_mut().entity_mut(world_player_entity).add_child(child);

        app.add_systems(Update, setup_animation);
        app.update();

        let entity = app.world().entity(child);

        assert!(
            entity.get::<AnimationGraphHandle>().is_some(),
            "Expected AnimationGraphHandle to be inserted"
        );

        assert!(
            entity.get::<AnimationTransitions>().is_some(),
            "Expected AnimationTransitions to be inserted"
        );
    }
    #[test]
    fn test_animation_update_system() {
        let mut app = App::new();
        app.init_resource::<Time>();

        let graph_handle = weak_handle!("f2c9d4a1-40d2-41dc-a9aa-5d489d2a43b5");
        let animation_indices = vec![
            AnimationNodeIndex::new(0),  // Idle
            AnimationNodeIndex::new(1),  // Idle2
            AnimationNodeIndex::new(2),  // Walking
            AnimationNodeIndex::new(3),  // Sprinting
        ];

        let mut animations_map = HashMap::new();
        animations_map.insert(
            "TestChar".to_string(),
            (graph_handle.clone(), animation_indices.clone()),
        );

        app.insert_resource(LoadedAssets {
            characters: HashMap::new(),
            animations: animations_map,
            environments: vec![],
        });

        let player_entity = app.world_mut().spawn((
            Character {
                name: "TestChar".to_string(),
                ..default()
            },
            WorldPlayer {
                state: WorldPlayerState::Idle,
                displayed_character: Character {
                    name: "TestChar".to_string(),
                    ..default()
                },
                ..default()
            },
        )).id();

        let anim_entity = app.world_mut().spawn((
            AnimationPlayer::default(),
            AnimationTransitions::new(),
        )).id();

        app.add_systems(Update, update_animation);

        app.update();

        app.world_mut().entity_mut(player_entity)
            .get_mut::<WorldPlayer>().unwrap().state = WorldPlayerState::Idle;

        app.update();

        let animation_player = app
            .world()
            .entity(anim_entity)
            .get::<AnimationPlayer>()
            .expect("Expected AnimationPlayer component");
        
        assert!(
            animation_player.playing_animations().any(|(idx, _)| idx.index() == 0),
            "Expected idle animation to be playing"
        );

        app.world_mut().entity_mut(player_entity)
            .get_mut::<WorldPlayer>().unwrap().state = WorldPlayerState::Walking;

        app.update();

        let animation_player = app
            .world()
            .entity(anim_entity)
            .get::<AnimationPlayer>()
            .expect("Expected AnimationPlayer component");

        assert!(
            animation_player.playing_animations().any(|(idx, _)| idx.index() == 2),
            "Expected walking animation to be playing"
        );

        app.world_mut().entity_mut(player_entity)
            .get_mut::<WorldPlayer>().unwrap().state = WorldPlayerState::Sprinting;

        app.update();

        let animation_player = app
            .world()
            .entity(anim_entity)
            .get::<AnimationPlayer>()
            .expect("Expected AnimationPlayer component");

        assert!(
            animation_player.playing_animations().any(|(idx, _)| idx.index() == 3),
            "Expected sprinting animation to be playing"
        );
    }


}