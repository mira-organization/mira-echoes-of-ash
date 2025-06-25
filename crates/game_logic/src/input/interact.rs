use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::inventory::{ItemSensor, NearbyItem, WorldItem};
use game_system::models::logic::WorldPlayer;
use game_system::utils::convert;

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_systems(Update, (detect_nearby_item_system, pickup_item_system)
            .run_if(in_state(GameState::InGame))
        );
    }
}

#[coverage(off)]
fn detect_nearby_item_system(
    mut nearby: ResMut<NearbyItem>,
    mut collision_events: EventReader<CollisionEvent>,
    sensors: Query<&ItemSensor>,
    players: Query<Entity, With<WorldPlayer>>,
) {
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
                    nearby.0 = Some(sensor.0);
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                if let Ok(sensor) = sensors.get(sensor_entity) {
                    if nearby.0 == Some(sensor.0) {
                        nearby.0 = None;
                    }
                }
            }
        }
    }

    fn extract_sensor_and_other(
        event: &CollisionEvent,
        sensors: &Query<&ItemSensor>,
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

#[coverage(off)]
fn pickup_item_system(
    input: Res<ButtonInput<KeyCode>>,
    mut nearby: ResMut<NearbyItem>,
    mut commands: Commands,
    world_items: Query<&WorldItem>,
    general_config: Res<ConfigService>,
) {
    let interact_key = convert(general_config.input_config.player_interact.as_str())
        .expect("Fetch key for (interact) was failed!");
    
    if input.just_pressed(interact_key) {
        if let Some(entity) = nearby.0 {
            if let Ok(world_item) = world_items.get(entity) {
                debug!("Item '{}' collected!", world_item.item.name);
                nearby.0 = None;
                commands.entity(entity).despawn();
            }
        }
    }
}