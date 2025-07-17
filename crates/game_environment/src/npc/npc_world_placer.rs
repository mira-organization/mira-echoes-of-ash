use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::dynamics::{Damping, LockedAxes, RigidBody, Velocity};
use bevy_rapier3d::geometry::{ActiveEvents, Collider, CollisionGroups, Group, Sensor};
use game_system::app_state::GameState;
use game_system::models::environment::CurrentEnvironment;
use game_system::models::GROUP_CHARACTER_COLLIDER;
use game_system::models::npcs::{NpcData, NpcSensor};
use game_system::save_info::LoadedAssets;

#[derive(Event)]
pub struct NpcSpawnEvent;

pub struct NpcWorldPlacer;

impl Plugin for NpcWorldPlacer {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_event::<NpcSpawnEvent>();
        app.add_systems(OnEnter(GameState::InGame), request_npc_spawn);
        app.add_systems(PostUpdate, load_to_world
            .run_if(resource_exists::<CurrentEnvironment>
            .and(resource_exists::<LoadedAssets>))
            .after(request_npc_spawn));
    }
}

/// Sends an [`NpcSpawnEvent`] to request spawning of all NPCs defined in the current environment.
///
/// This function emits a single `NpcSpawnEvent` using the [`EventWriter`] API. It is intended to be called
/// when the system decides that NPCs should be spawned in the world (for example, after a scene is loaded).
///
/// # Parameters
/// - `event_writer`: Writer for the [`NpcSpawnEvent`] which will be dispatched.
///
/// # Side Effects
/// - Emits a spawn event that triggers [`load_to_world`] system to spawn NPCs.
#[coverage(off)]
fn request_npc_spawn(mut event_writer: EventWriter<NpcSpawnEvent>) {
    event_writer.write(NpcSpawnEvent);
}

/// Handles [`NpcSpawnEvent`]s and spawns all NPCs into the world at their predefined locations.
///
/// This function iterates over all non-player characters (NPCs) defined in [`CurrentEnvironment`],
/// checks if they have at least one location, and spawns them into the game world using [`spawn_fake_player`].
///
/// # Parameters
/// - `event_reader`: Reader for [`NpcSpawnEvent`] to listen for spawn requests.
/// - `current_environment`: Reference to the current environment data, including all NPC definitions.
/// - `commands`: Mutable reference to [`Commands`] for spawning new entities.
/// - `assets`: Reference to [`LoadedAssets`] containing character models and animation data.
///
/// # Side Effects
/// - Spawns NPC entities and corresponding sensor colliders into the Bevy world.
/// - Logs warnings if NPCs have no spawn location.
#[coverage(off)]
fn load_to_world(
    mut event_reader: EventReader<NpcSpawnEvent>,
    current_environment: Res<CurrentEnvironment>,
    mut commands: Commands,
    assets: Res<LoadedAssets>,
) {
    for _ in event_reader.read() {
        for (_, npc_data) in current_environment.area.non_player_characters.iter() {
            debug!("Loading NPC data for {}", npc_data.name);
            if npc_data.locations.is_empty() {
                warn!("No NPC location for {}", npc_data.name);
                continue;
            }

            if let Some(location) = npc_data.locations.first() {
                spawn_fake_player(
                    &mut commands,
                    &assets,
                    &npc_data,
                    &Transform::from_xyz(location.x, location.y, location.z));
            }
        }
    }   
}

/// Spawns an individual NPC character in the game world, including its physics components and collision sensor.
///
/// This function finds the correct 3D scene handle for the NPC by matching its name in [`LoadedAssets`],
/// and attaches physics components (rigid body, collider, damping) along with a separate sensor collider entity
/// for detecting item interactions.
///
/// # Parameters
/// - `commands`: Mutable reference to [`Commands`] used to spawn entities.
/// - `assets`: Reference to [`LoadedAssets`] to find character scene handles.
/// - `npc_data`: Reference to [`NpcData`] defining NPC name and metadata.
/// - `transform`: Transform specifying the spawn position.
///
/// # Side Effects
/// - Adds an NPC scene entity to the world.
/// - Adds a separate sensor entity linked to the NPC for collision detection.
#[coverage(off)]
fn spawn_fake_player(
    commands: &mut Commands,
    assets: &LoadedAssets,
    npc_data: &NpcData,
    transform: &Transform,
) {
    debug!("Spawning NPC player at {:?}", transform);

    let mut scene = Default::default();
    for (key, handle) in assets.characters.clone() {
        if key.eq_ignore_ascii_case(&npc_data.name) {
            scene = handle;
            break;
        }
    }

    let parent = commands.spawn((
        Name::new(format!("NPC-{}",  npc_data.name)),
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
        SceneRoot(scene.clone()),
        Transform::from_translation(transform.translation),
        npc_data.clone(),
    )).id();

    commands.spawn((
        RigidBody::Fixed,
        Collider::ball(0.25 * 8.0),
        Sensor,
        Transform::from_translation(transform.translation),
        CollisionGroups::new(GROUP_CHARACTER_COLLIDER, Group::all()),
        ActiveEvents::COLLISION_EVENTS,
        NpcSensor(parent),
    ));
}