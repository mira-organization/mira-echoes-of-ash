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

/// Detects nearby entities of a generic type using sensors and collision events.
///
/// This system listens for collision start and stop events between sensor entities and the player entity.
/// When the player enters a sensor's collision area, the nearby target resource is updated to the sensor's target entity.
/// When the player leaves the sensor's collision area, the nearby target resource is cleared if it matched.
///
/// # Type Parameters
/// * `TSensor` - The component type representing the sensor entity. Must implement `SensorTarget` and be a component.
/// * `TRes` - The resource type used to store the currently nearby target. Must implement `NearbyTarget` and be a Bevy resource.
///
/// # Parameters
/// * `res` - Mutable resource of type `TRes` to update the current nearby target.
/// * `collision_events` - Event reader for `CollisionEvent's triggered by physics collisions.
/// * `sensors` - Query for all sensor components in the world.
/// * `players` - Query to get the player entity, filtered by the ` WorldPlayer ` component.
///
/// # Behavior
/// - Only the single player entity is considered.
/// - On collision start, if the other collider is the player and one collider is a sensor, sets the resource to the sensor's target entity.
/// - On collision stop, if the player leaves a sensor, clears the resource if it still points to that sensor's target entity.
///
/// # Notes
/// The helper function `extract_sensor_and_other` extracts the sensor entity and the other entity from a collision event.
///
/// This function must be registered as a Bevy system with appropriate generic parameters.
#[coverage(off)]
fn detect_nearby_generic<TSensor: Component, TRes: NearbyTarget + bevy::prelude::Resource>(
    mut res: ResMut<TRes>,
    mut collision_events: EventReader<CollisionEvent>,
    sensors: Query<(Entity, &TSensor)>,
    players: Query<Entity, With<WorldPlayer>>,
)
where
    TSensor: SensorTarget + Component,
    TRes: NearbyTarget + Resource,
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
                if let Ok((_, sensor)) = sensors.get(sensor_entity) {
                    res.set(Some(sensor.target_entity()));
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                if let Ok((_, sensor)) = sensors.get(sensor_entity) {
                    let still_active = sensors.iter().any(|(entity, s)| {
                        entity != sensor_entity &&
                            s.target_entity() == sensor.target_entity() &&
                            true
                    });

                    if !still_active && res.get() == Some(sensor.target_entity()) {
                        res.set(None);
                    }
                }
            }
        }
    }

    fn extract_sensor_and_other<TSensor: Component>(
        event: &CollisionEvent,
        sensors: &Query<(Entity, &TSensor)>,
    ) -> Option<(Entity, Entity)> {
        match event {
            CollisionEvent::Started(e1, e2, _) | CollisionEvent::Stopped(e1, e2, _) => {
                if sensors.get(*e1).is_ok() {
                    Some((*e1, *e2))
                } else if sensors.get(*e2).is_ok() {
                    Some((*e2, *e1))
                } else {
                    None
                }
            }
        }
    }
}

/// System that detects nearby items by processing collision events with item sensors.
///
/// Updates the `NearbyItem` resource to reflect the current item near the player.
#[coverage(off)]
fn detect_nearby_item_system(
    nearby: ResMut<NearbyItem>,
    events: EventReader<CollisionEvent>,
    sensors: Query<(Entity, &ItemSensor)>,
    players: Query<Entity, With<WorldPlayer>>,
) {
    detect_nearby_generic::<ItemSensor, NearbyItem>(nearby, events, sensors, players);
}


/// System that detects nearby NPCs by processing collision events with NPC sensors.
///
/// Updates the `NearbyNpc` resource to reflect the current NPC near the player.
#[coverage(off)]
fn detect_nearby_npc_system(
    nearby: ResMut<NearbyNpc>,
    events: EventReader<CollisionEvent>,
    sensors: Query<(Entity, &NpcSensor)>,
    players: Query<Entity, With<WorldPlayer>>,
) {
    detect_nearby_generic::<NpcSensor, NearbyNpc>(nearby, events, sensors, players);
}

/// System that allows the player to pick up a nearby item when pressing the interacted key.
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