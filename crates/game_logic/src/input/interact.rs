use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::inventory::{ItemSensor, NearbyItem, WorldItem};
use game_system::models::logic::{NearbyTarget, SensorTarget, WorldPlayer};
use game_system::models::npcs::{NearbyNpc, NpcSensor};
use game_system::save_info::SaveInfo;
use game_system::utils::convert;

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_systems(Update, (
            detect_nearby_npc_system,
            detect_nearby_item_system,
            pickup_item_system
        ).run_if(in_state(GameState::InGame))
        );
    }
}

fn detect_nearby_generic<TSensor: Component, TRes: NearbyTarget + bevy::prelude::Resource>(
    mut res: ResMut<TRes>,
    mut collision_events: EventReader<CollisionEvent>,
    sensors: Query<&TSensor>,
    players: Query<Entity, With<WorldPlayer>>,
)
where
    TSensor: SensorTarget,
{
    let Ok(player_entity) = players.single() else { return; };

    for event in collision_events.read() {
        let (sensor_entity, other) = match extract_sensor_and_other(event, &sensors) {
            Some(pair) => pair,
            None => continue,
        };

        if other != player_entity {
            continue;
        }

        match event {
            CollisionEvent::Started(_, _, _) => {
                if let Ok(sensor) = sensors.get(sensor_entity) {
                    res.set(Some(sensor.target_entity()));
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                if let Ok(sensor) = sensors.get(sensor_entity) {
                    if res.get() == Some(sensor.target_entity()) {
                        res.set(None);
                    }
                }
            }
        }
    }

    fn extract_sensor_and_other<TSensor: Component>(
        event: &CollisionEvent,
        sensors: &Query<&TSensor>,
    ) -> Option<(Entity, Entity)> {
        match event {
            CollisionEvent::Started(e1, e2, _) | CollisionEvent::Stopped(e1, e2, _) => {
                if sensors.contains(*e1) {
                    Some((*e1, *e2))
                } else if sensors.contains(*e2) {
                    Some((*e2, *e1))
                } else {
                    None
                }
            }
        }
    }
}

fn detect_nearby_item_system(
    nearby: ResMut<NearbyItem>,
    events: EventReader<CollisionEvent>,
    sensors: Query<&ItemSensor>,
    players: Query<Entity, With<WorldPlayer>>,
) {
    detect_nearby_generic::<ItemSensor, NearbyItem>(nearby, events, sensors, players);
}

fn detect_nearby_npc_system(
    nearby: ResMut<NearbyNpc>,
    events: EventReader<CollisionEvent>,
    sensors: Query<&NpcSensor>,
    players: Query<Entity, With<WorldPlayer>>,
) {
    detect_nearby_generic::<NpcSensor, NearbyNpc>(nearby, events, sensors, players);
}

/// System that allows the player to pick up a nearby item when pressing the interact key.
///
/// If an item is near the player and the correct key is pressed, the item and its sensor
/// collider are removed from the world, and the `NearbyItem` resource is cleared.
///
/// # Parameters
/// - `input`: Player input state (keyboard).
/// - `nearby`: The currently nearby item, if any.
/// - `commands`: Command buffer for despawning entities.
/// - `world_items`: Query for world item data.
/// - `sensors`: Query for all `ItemSensor` components.
/// - `general_config`: Access to keybindings and game configuration.
#[coverage(off)]
fn pickup_item_system(
    input: Res<ButtonInput<KeyCode>>,
    mut nearby: ResMut<NearbyItem>,
    mut commands: Commands,
    world_items: Query<&WorldItem>,
    sensors: Query<(Entity, &ItemSensor)>,
    general_config: Res<ConfigService>,
    mut inventory: ResMut<SaveInfo>,
) {
    let interact_key = convert(general_config.input_config.player_interact.as_str())
        .expect("Fetch key for (interact) was failed!");

    if input.just_pressed(interact_key) {
        if let Some(entity) = nearby.0 {
            if let Ok(world_item) = world_items.get(entity) {
                debug!("Item '{}' collected!", world_item.item.name);
                
                commands.entity(entity).despawn();
                
                for (sensor_entity, sensor) in &sensors {
                    if sensor.0 == entity {
                        commands.entity(sensor_entity).despawn();
                        break;
                    }
                }

                let item = world_item.item.clone();
                let name = item.name.clone();
                
                if let Some(existing_item) = inventory
                    .items
                    .iter_mut()
                    .find(|i| i.name == name)
                {
                    existing_item.value += item.value;
                } else {
                    inventory.items.push(item);
                }
                nearby.0 = None;
            }
        }
    }
}