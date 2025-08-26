use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::prelude::*;
use game_core::collider_groups::GROUP_CHARACTER_COLLIDER;
use game_core::entities::animation::AnimKey;
use game_core::entities::non_player::{NpcData, NpcSensor, WorldNpc};
use game_core::events::non_player_events::NpcPreSpawnEvent;
use game_core::loading::LoadedAssets;
use game_core::states::AppState;
use game_core::world::environment::CurrentEnvironment;

pub struct NpcWorldService;

impl Plugin for NpcWorldService {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::PostLoad), pre_spawn_npc);
        app.add_systems(PostUpdate, load_to_world
            .run_if(resource_exists::<CurrentEnvironment>
                .and(resource_exists::<CurrentEnvironment>)
            ));
    }
}

/// Broadcasts a single `NpcPreSpawnEvent`.
///
/// This lightweight system kicks off NPC spawning. Run it once after loading an
/// area/level or whenever you want to (re)populate the world with NPCs that are
/// defined in the current environment.
///
/// ### Behavior
/// - Emits exactly one `NpcPreSpawnEvent`.
///
/// ### Scheduling
/// Typical usage is to run this on `OnEnter` of a state or right after you load
/// the `CurrentEnvironment`.
#[coverage(off)]
fn pre_spawn_npc(mut event_writer: EventWriter<NpcPreSpawnEvent>) {
    event_writer.write(NpcPreSpawnEvent);
}


/// Consumes `NpcPreSpawnEvent` and spawns all NPCs of the current area
/// into the world at their first declared location.
///
/// ### Behavior
/// - For each `NpcPreSpawnEvent` read, iterates over
///   `current_environment.area.non_player_characters`.
/// - Skips NPCs without any `locations` and logs a warning.
/// - For NPCs with at least one location, calls [`generate_non_player`]
///   using the *first* location only.
/// - The spawned NPC also gets an associated dialog sensor (see
///   [`generate_non_player`]).
///
/// ### Parameters
/// - `event_reader`: reads pending `NpcPreSpawnEvent`s.
/// - `current_environment`: provides NPC data for the active area.
/// - `commands`: used to spawn entities and components.
/// - `assets`: lookup table for scene/entity handles used by NPCs.
///
/// ### Notes
/// - This system does **not** path or move NPCs; it only places them at
///   their initial position. Later behavior is handled elsewhere.
/// - If there are multiple events queued, NPCs will be spawned for each read
///   event (idempotence is up to the caller).
#[coverage(off)]
fn load_to_world(
    mut event_reader: EventReader<NpcPreSpawnEvent>,
    current_environment: Res<CurrentEnvironment>,
    mut commands: Commands,
    assets: Res<LoadedAssets>
) {
    for _ in event_reader.read() {
        for (_, npc_data) in current_environment.area.non_player_characters.iter() {
            if npc_data.locations.is_empty() {
                warn!("NPC [ {} ] has no locations defined", npc_data.name);
                continue;
            }

            if let Some(location) = npc_data.locations.first() {
                generate_non_player(&mut commands, &assets, &npc_data, &Transform::from_xyz(location.x, location.y, location.z));
            }
        }
    }
}

/// Spawns a single NPC entity and a companion dialog sensor at the given transform.
///
/// ### Behavior
/// - Resolves a scene/entity handle by case-insensitive match against `npc_data.name`
///   from `assets.entities`. If none is found, `SceneRoot` is initialized with
///   the default handle (ensure your asset map contains a matching entry).
/// - Spawns the **NPC parent** entity with:
///   - `Name = "NPC-{name}"`
///   - `NoFrustumCulling`
///   - `RigidBody::Dynamic` with linear & angular damping
///   - Locked X/Z rotation via `LockedAxes`
///   - Capsule `Collider` suitable for a biped
///   - `CollisionGroups` targeting `GROUP_CHARACTER_COLLIDER`
///   - `SceneRoot(non_player_scene)`
///   - `Transform` set to `transform.translation`
///   - A copy of `npc_data`
///   - `WorldNpc { displayed_npc: npc_data.clone(), ..default() }`
///   - `AnimKey(npc_data.name.clone())`
/// - Spawns a separate **dialog sensor** entity with:
///   - `Name = "NPC-Dialog-Sensor"`
///   - `RigidBody::Fixed`
///   - Spherical `Collider::ball(2.0)` marked as `Sensor`
///   - `ActiveEvents::COLLISION_EVENTS` to receive collision events
///   - Same world `Transform` (not parented) as the NPC
///   - `CollisionGroups` compatible with character colliders
///   - `NpcSensor(parent_entity)` linking back to the NPC
///
/// ### Parameters
/// - `commands`: command buffer used to create entities/components.
/// - `assets`: asset index used to resolve the NPC's scene handle.
/// - `npc_data`: the logical data for the NPC being spawned (name, config, etc.).
/// - `transform`: initial world transform/position for the NPC and its sensor.
///
/// ### Returns
/// - This function does not return an entity; it performs the spawning side effects
///   directly via `commands`.
///
/// ### Gotchas
/// - If the NPC name does not exist in `assets.entities`, the default scene handle
///   is used, which typically results in an empty or missing visual. Keep the asset
///   map in sync with your NPC definitions.
/// - The sensor is spawned as a separate entity at the same position, not as a child.
///   If you need it to follow the NPC transform automatically, consider parenting it.
#[coverage(off)]
fn generate_non_player(
    commands: &mut Commands,
    assets: &LoadedAssets,
    npc_data: &NpcData,
    transform: &Transform,
) {
    debug!("Generating non player at location [ {:?} ]", transform.translation);

    let mut non_player_scene = Default::default();
    for (key, handle) in assets.entities.clone() {
        if key.eq_ignore_ascii_case(&npc_data.name) {
            non_player_scene = handle;
            break;
        }
    }

    let parent_entity = commands.spawn((
        Name::new(format!("NPC-{}", npc_data.name)),
        NoFrustumCulling,
        RigidBody::Dynamic,
        Velocity::default(),
        Damping {
            angular_damping: 2.0,
            linear_damping: 2.0,
        },
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Collider::capsule(Vec3::new(0.0, 0.2, 0.0), Vec3::new(0.0, 1.6, 0.0), 0.2),
        CollisionGroups::new(GROUP_CHARACTER_COLLIDER, Group::all()),
        SceneRoot(non_player_scene),
        Transform::from_translation(transform.translation),
        npc_data.clone(),
        WorldNpc {
            displayed_npc: npc_data.clone(),
            ..default()
        },
        AnimKey(npc_data.name.clone())
    )).id();

    commands.spawn((
        Name::new("NPC-Dialog-Sensor"),
        RigidBody::Fixed,
        Collider::ball(2.0),
        Sensor,
        Transform::from_translation(transform.translation),
        CollisionGroups::new(GROUP_CHARACTER_COLLIDER, Group::all()),
        ActiveEvents::COLLISION_EVENTS,
        NpcSensor(parent_entity)
    ));
}