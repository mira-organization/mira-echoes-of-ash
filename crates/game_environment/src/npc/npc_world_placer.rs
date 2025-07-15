use bevy::prelude::*;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{ActiveEvents, Collider, CollisionGroups, Group, Sensor};
use game_system::app_state::GameState;
use game_system::models::environment::CurrentEnvironment;
use game_system::models::GROUP_ITEMS_COLLIDER;
use game_system::models::inventory::ItemSensor;
use game_system::models::npcs::NpcData;

#[derive(Event)]
pub struct NpcSpawnEvent;

pub struct NpcWorldPlacer;

impl Plugin for NpcWorldPlacer {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_event::<NpcSpawnEvent>();
        app.add_systems(OnEnter(GameState::InGame), request_npc_spawn);
        app.add_systems(PostUpdate, load_to_world.run_if(resource_exists::<CurrentEnvironment>));
    }
}

#[coverage(off)]
fn request_npc_spawn(mut event_writer: EventWriter<NpcSpawnEvent>) {
    event_writer.write(NpcSpawnEvent);
}

#[coverage(off)]
fn load_to_world(
    mut event_reader: EventReader<NpcSpawnEvent>,
    current_environment: Res<CurrentEnvironment>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    for _ in event_reader.read() {
        info!("!!!");
        for (_, npc_data) in current_environment.area.non_player_characters.iter() {
            info!("Loading NPC data for {}", npc_data.name);
            if npc_data.locations.is_empty() {
                warn!("No NPC location for {}", npc_data.name);
                continue;
            }

            if let Some(location) = npc_data.locations.first() {
                spawn_fake_player(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &npc_data,
                    &Transform::from_xyz(location.x, location.y, location.z));
            }
        }
    }   
}

#[coverage(off)]
fn spawn_fake_player(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    npc_data: &NpcData,
    transform: &Transform,
) {
    info!("Spawning NPC player at {:?}", transform);

    let mesh = meshes.add(Mesh::from(Cuboid::new(0.3, 2.5, 0.3)));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.9, 0.9, 0.8),
        emissive: LinearRgba::from(Color::srgba(0.9, 0.9, 0.9, 0.8)),
        unlit: false,
        ..default()
    });

    let parent = commands.spawn((
        Name::new(format!("NPC-{}",  npc_data.name)),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(transform.translation),
    )).id();

    commands.spawn((
        RigidBody::Fixed,
        Collider::ball(0.25 * 8.0),
        Sensor,
        Transform::from_translation(transform.translation),
        CollisionGroups::new(GROUP_ITEMS_COLLIDER, Group::all()),
        ActiveEvents::COLLISION_EVENTS,
        ItemSensor(parent),
    ));
}