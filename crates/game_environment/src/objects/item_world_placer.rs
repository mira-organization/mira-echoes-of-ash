use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody, Sensor};
use game_system::app_state::GameState;
use game_system::app_state::GameState::LoadGameAssets;
use game_system::models::environment::CurrentEnvironment;
use game_system::models::GROUP_ITEMS_COLLIDER;
use game_system::models::inventory::{ItemSensor, WorldItem};

#[derive(Component)]
struct FloatingRotatingItem {
    pub base_y: f32,
    pub speed: f32,
    pub amplitude: f32,
    pub rotation_speed: f32,
}

pub struct ItemWorldPlacer;

impl Plugin for ItemWorldPlacer {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LoadGameAssets), load_to_world);
        app.add_systems(Update, animate_floating_items.run_if(in_state(GameState::InGame)));
    }
}

/// Loads item entities into the world based on the current environment.
///
/// This function checks the `CurrentEnvironment` resource for items associated with the current area.
/// Each item is spawned into the world at its configured position using [`spawn_item_cube`].
///
/// # Parameters
/// - `current_environment`: The current game environment and area context.
/// - `meshes`: Asset storage for meshes.
/// - `materials`: Asset storage for materials.
/// - `commands`: Command buffer to spawn new entities.
#[coverage(off)]
fn load_to_world(
    current_environment: Res<CurrentEnvironment>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    if let Some(item_list) = current_environment.area.items.get(current_environment.area.name.as_str()) {
        for world_item in item_list.iter() {
            debug!("place item {} at: {:?}", world_item.item.name.clone(), world_item.location.clone());
            let location = world_item.location.clone();
            spawn_item_cube(
                &mut commands,
                &world_item,
                &Transform::from_xyz(location.x, location.y, location.z),
                &mut meshes,
                &mut materials,
            );
        }
    }
}

/// Applies floating and rotating animations to all floating items in the world.
///
/// Each floating item moves up and down sinusoidally and rotates around the Y axis.
///
/// # Parameters
/// - `time`: Global time resource for calculating animation progression.
/// - `query`: Query for all floating item entities and their transforms.
#[coverage(off)]
fn animate_floating_items(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FloatingRotatingItem)>
) {
    let t = time.elapsed_secs();
    for (mut transform, float) in &mut query {
        let offset = float.amplitude * (t * float.speed).sin();
        transform.translation.y = float.base_y + offset;
        transform.rotate_y(float.rotation_speed * time.delta_secs());
    }
}

/// Spawns a visual item cube and an associated collider in the world.
///
/// The item's rarity determines the visual appearance. A corresponding sensor collider
/// is created to detect proximity-based interactions.
///
/// # Parameters
/// - `commands`: The command buffer used to spawn the entities.
/// - `world_item`: The item data and position in the world.
/// - `transform`: The transform indicating where to spawn the item.
/// - `meshes`: The asset storage for mesh resources.
/// - `materials`: The asset storage for material resources.
#[coverage(off)]
fn spawn_item_cube(
    commands: &mut Commands,
    world_item: &WorldItem,
    transform: &Transform,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let emissive = match world_item.item.rarity.as_str() {
        "normal"    => Color::srgb(0.8, 0.8, 0.8),
        "rare"      => Color::srgb(0.3, 0.3, 0.7),
        "mythic"    => Color::srgb(0.4, 0.0, 0.7),
        "legendary" => Color::srgb(1.0, 0.84, 0.0),
        _           => Color::WHITE,
    };

    // Mesh und Material
    let mesh = meshes.add(Mesh::from(Cuboid::new(0.15, 0.15, 0.15)));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.9, 0.9, 0.8),
        emissive: LinearRgba::from(emissive),
        unlit: false,
        ..default()
    });

    // Item-Visual Entity
    let parent = commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        FloatingRotatingItem {
            base_y: transform.translation.y,
            speed: 2.0,
            amplitude: 0.1,
            rotation_speed: 1.0,
        },
        Transform::from_translation(transform.translation),
        world_item.clone(),
    )).id();
    
    commands.spawn((
        RigidBody::Fixed,
        Collider::ball(0.15 * 8.0),
        Sensor,
        Transform::from_translation(transform.translation),
        CollisionGroups::new(GROUP_ITEMS_COLLIDER, Group::all()),
        ActiveEvents::COLLISION_EVENTS,
        ItemSensor(parent),
    ));
}