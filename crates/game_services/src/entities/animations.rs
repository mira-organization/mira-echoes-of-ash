use std::time::Duration;
use bevy::prelude::*;
use bevy::window::WindowCloseRequested;
use game_core::entities::animation::{AnimIntent, AnimKey, AnimOwner, AnimPlayback, AnimRole, AnimSchema, IdleVariantTimer, Locomotion};
use game_core::entities::character::Character;
use game_core::entities::non_player::{WorldNpc, WorldNpcState};
use game_core::entities::player::{WorldPlayer, WorldPlayerState};
use game_core::loading::LoadedAssets;
use game_core::states::{AppState, InGameStates};

pub struct AnimationsLogic;

impl Plugin for AnimationsLogic {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bind_animations_to_entities,
                (compose_player_anim_intent, compose_npc_anim_intent),
                apply_anim_intent,
            )
                .chain()
                .run_if(in_state(AppState::InGame(InGameStates::Game)))
                .run_if(resource_exists::<LoadedAssets>)
                .run_if(not(on_event::<AppExit>))
                .run_if(not(on_event::<WindowCloseRequested>)),
        );
    }
}

/// Binds animation data to newly added `AnimationPlayer` children by walking up the
/// transform hierarchy to find their logical owner and animation key.
///
/// This system:
/// - Ascends via `ChildOf` from the `AnimationPlayer` child to the first ancestor that has an [`AnimKey`].
/// - Determines the animation role by checking the owner for `WorldPlayer` / `WorldNpc` (`Player` > `Npc` > `Prop`).
/// - Looks up the animation graph and clip table in `LoadedAssets.animations` using the `AnimKey`.
/// - Starts the base idle clip and attaches all runtime animation components to the child:
///   `AnimationGraphHandle`, `AnimationTransitions`, [`AnimOwner`], [`AnimRole`], [`AnimSchema`],
///   [`AnimIntent`], [`IdleVariantTimer`], and [`AnimPlayback`].
///
/// Runs on `Added<AnimationPlayer>`; typically scheduled before any intent-composer and apply systems.
///
/// # Parameters
/// - `commands`: To insert the runtime animation components onto the child entity.
/// - `loaded_assets`: Asset registry providing graphs and clip handles keyed by `AnimKey`.
/// - `new_players`: Newly spawned/attached `AnimationPlayer` components to bind.
/// - `parents`: Used to walk from the child up to its owner.
/// - `characters`: Owner entities that carry an [`AnimKey`] (query excludes `AnimationPlayer`).
/// - `world_players`: Presence indicates the owner is the player (role = `Player`).
/// - `world_non_players`: Presence indicates the owner is an NPC (a role = `Npc`).
#[coverage(off)]
fn bind_animations_to_entities(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    mut new_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    characters: Query<(Entity, &AnimKey), Without<AnimationPlayer>>,
    world_players: Query<(), With<WorldPlayer>>,
    world_non_players: Query<(), With<WorldNpc>>,
) {
    for (child_entity, mut animation_player) in new_players.iter_mut() {
        let mut current = child_entity;
        while let Ok(parent) = parents.get(current) {
            current = parent.parent();

            if let Ok((_owner, key)) = characters.get(current) {
                let role = if world_players.get(current).is_ok() {
                    AnimRole::Player
                } else if world_non_players.get(current).is_ok() {
                    AnimRole::Npc
                } else {
                    AnimRole::Prop
                };

                if let Some((graph, clips)) = loaded_assets.animations.get(key.0.as_str()) {
                    let schema = AnimSchema {
                        idle_base: 0,
                        idle_variants: &[1],
                        walk: 2,
                        run: 3,
                        slow_walk: 4,
                        jump: Some(5),
                    };

                    let mut transitions = AnimationTransitions::new();
                    transitions.play(&mut animation_player, clips[schema.idle_base], Duration::ZERO).repeat();

                    commands.entity(child_entity)
                        .insert(AnimationGraphHandle(graph.clone()))
                        .insert(transitions)
                        .insert(AnimOwner(current))
                        .insert(role)
                        .insert(schema)
                        .insert(AnimIntent::default())
                        .insert(IdleVariantTimer(Timer::from_seconds(20.0, TimerMode::Repeating)))
                        .insert(AnimPlayback::default());
                }
                break;
            }
        }
    }
}

/// Composes canonical animation intent **for player-controlled characters**.
///
/// Reads the `WorldPlayer` state from the owner entity (referenced via [`AnimOwner`]) and
/// writes the corresponding locomotion value into [`AnimIntent`] on the `AnimationPlayer` child.
///
/// Must run **before** the applied system so the new intent is consumed in the same frame.
///
/// # Parameters
/// - `intents`: `(AnimIntent, AnimOwner, AnimRole)` on animation children to write into (filtered to `Player`).
/// - `players`: Provides access to `WorldPlayer` on owner entities.
#[coverage(off)]
fn compose_player_anim_intent(
    mut intents: Query<(&mut AnimIntent, &AnimOwner, &AnimRole)>,
    players: Query<&WorldPlayer>,
) {
    for (mut intent, owner, role) in &mut intents {
        if *role != AnimRole::Player { continue; }
        if let Ok(player) = players.get(owner.0) {
            intent.locomotion = Some(match player.state {
                WorldPlayerState::Idle      => Locomotion::Idle,
                WorldPlayerState::Walking   => Locomotion::Walk,
                WorldPlayerState::Sprinting => Locomotion::Run,
                WorldPlayerState::Jumping   => Locomotion::Jump,
                WorldPlayerState::SlowWalk  => Locomotion::SlowWalk,
            });
        }
    }
}

/// Composes a canonical animation intent **for NPCs**.
///
/// Reads the `WorldNpc` state from the owner entity (referenced via [`AnimOwner`]) and
/// writes the corresponding locomotion value into [`AnimIntent`] on the `AnimationPlayer` child.
///
/// Must run **before** the applied system so the new intent is consumed in the same frame.
///
/// # Parameters
/// - `intents`: `(AnimIntent, AnimOwner, AnimRole)` on animation children to write into (filtered to `Npc`).
/// - `non_players`: Provides access to `WorldNpc` on owner entities.
#[coverage(off)]
fn compose_npc_anim_intent(
    mut intents: Query<(&mut AnimIntent, &AnimOwner, &AnimRole)>,
    non_players: Query<&WorldNpc>,
) {
    for (mut intent, owner, role) in &mut intents {
        if *role != AnimRole::Npc { continue; }
        if let Ok(npc) = non_players.get(owner.0) {
            intent.locomotion = Some(match npc.state {
                WorldNpcState::Idle      => Locomotion::Idle,
                WorldNpcState::Walking   => Locomotion::Walk,
                WorldNpcState::Sprinting => Locomotion::Run,
            });
        }
    }
}

/// Applies the current [`AnimIntent`] to an `AnimationPlayer` using the entity’s [`AnimSchema`].
///
/// Behavior:
/// - Keeps the base idle looping and occasionally plays an idle **one-shot** variant controlled by
///   [`IdleVariantTimer`]. While a variant is active, it is allowed to finish unless locomotion
///   changes away from `Idle`, in which case the variant is canceled immediately.
/// - Switches to walk/run/jump with smooth transition times and appropriate repeat flags.
/// - Avoids replay-spam by caching the last clip index in [`AnimPlayback`] and skipping redundant plays.
///
/// **Order: ** Run after all composer systems (player/NPC/…) and after the binding system.
///
/// # Parameters
/// - `time`: For ticking the idle-variant timer.
/// - `loaded_assets`: Source of clip handles; looked up per owner.
/// - `characters`: Used to resolve the owner’s animation set (by `Character.name`) for clip lookup.
/// - `q`: Query over animation children, providing owner link, schema, intent, per-entity timer,
///        playback cache, and access to `AnimationPlayer`/`AnimationTransitions`.
///
/// # Notes
/// - This implementation expects the owner to have a `Character` with a `name` that maps into
///   `LoadedAssets.animations`. If you standardize on [`AnimKey`] instead, adjust the lookup
///   query accordingly.
#[coverage(off)]
fn apply_anim_intent(
    time: Res<Time>,
    loaded_assets: Res<LoadedAssets>,
    characters: Query<&Character>,
    mut q: Query<(
        &AnimOwner,
        &AnimSchema,
        &AnimIntent,
        &mut IdleVariantTimer,
        &mut AnimPlayback,
        &mut AnimationPlayer,
        &mut AnimationTransitions,
    )>,
) {
    for (owner, schema, intent, mut idle_timer, mut pb, mut player, mut transitions) in &mut q {
        let Ok(character) = characters.get(owner.0) else { continue; };
        let Some((_, clips)) = loaded_assets.animations.get(character.name.as_str()) else { continue; };

        idle_timer.0.tick(time.delta());

        let desired = intent.locomotion.unwrap_or(Locomotion::Idle);

        if desired != Locomotion::Idle && pb.one_shot_override.is_some() {
            pb.one_shot_override = None;
        }

        if desired == Locomotion::Idle {
            if let Some(variant_idx) = pb.one_shot_override {
                let variant_node = clips[variant_idx];

                let mut still_playing = false;
                let mut finished = false;
                for (node, active) in player.playing_animations_mut() {
                    if *node == variant_node {
                        still_playing = true;
                        finished = active.is_finished();
                        break;
                    }
                }

                if still_playing && !finished {
                    pb.current_index = Some(variant_idx);
                    continue;
                } else {
                    pb.one_shot_override = None;
                }
            }
        }

        let (target_index, repeat, transition_ms) = match desired {
            Locomotion::Idle => {
                if idle_timer.0.finished() && !schema.idle_variants.is_empty() {
                    let idx = schema.idle_variants[rand::random_range(0..schema.idle_variants.len())];
                    idle_timer.0.reset();
                    pb.one_shot_override = Some(idx);
                    (idx, false, 425)
                } else {
                    (schema.idle_base, true, 425)
                }
            }
            Locomotion::Walk => (schema.walk,  true, 450),
            Locomotion::Run  => (schema.run,   true, 550),
            Locomotion::Jump => (schema.jump.unwrap_or(schema.jump.unwrap_or(schema.idle_base)), false, 250),
            Locomotion::SlowWalk => (schema.slow_walk, true, 200)
        };

        let target_node = clips[target_index];

        if pb.current_index == Some(target_index) && player.is_playing_animation(target_node) {
            continue;
        }

        let dur = Duration::from_millis(transition_ms);
        let play = transitions.play(&mut player, target_node, dur);
        if repeat { play.repeat(); }

        pb.current_index = Some(target_index);
    }
}

