use std::collections::HashMap;
use std::f32::consts::PI;
use bevy::gltf::GltfNode;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::prelude::{AsyncSceneCollider, ComputedColliderShape, RigidBody, TriMeshFlags};
use serde_json::Value;
use game_system::app_state::GameState;
use game_system::models::environment::{CurrentAreaScenes, CurrentEnvironment, EffectSceneAssets, EnvironmentScene, LightData, LightType, WaitingForAreaAssets};
use game_system::save_info::LoadedAssets;

pub struct ReadyUpHandles;

impl Plugin for ReadyUpHandles {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadGameAssets), (pre_load_area, pre_load_gltf_assets));
        app.add_systems(Update, process_loaded_area.run_if(in_state(GameState::LoadGameAssets)));
        app.add_systems(Update, load_active_area_lights.run_if(in_state(GameState::PostLoad)));
        app.add_systems(OnEnter(GameState::PostLoad), load_active_area);
    }
}

/// Pre-loads the `.glb` file of the current area before it is fully loaded into the game world.
/// This ensures that the asset is available in the asset pipeline before rendering.
///
/// Parameters:
/// - `commands`: Bevy's command buffer used to insert resources.
/// - `asset_server`: The asset server responsible for loading assets asynchronously.
/// - `environment`: The currently active environment, containing the area name.
///
/// Behavior:
/// - Constructs the asset path using the current environment and area name.
/// - Requests the asset server to load the `.glb` file.
/// - Stores the loading handle in a `WaitingForAreaAssets` resource.
///
/// Logging:
/// - Outputs an informational log message indicating that the `.glb` file is being preloaded.
#[coverage(off)]
pub fn pre_load_area(mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     environment: Res<CurrentEnvironment>, mut assets_to_load: ResMut<LoadedAssets>
) {
    let path = format!("environments/{}/{}", environment.environment.name, environment.area.name);
    let glb_handle = asset_server.load::<Gltf>(path.as_str());
    commands.insert_resource(WaitingForAreaAssets(glb_handle.clone()));
    assets_to_load.environments.push(glb_handle.untyped().id());
    info!("Pre Loading glb [{:?}]", path);
}

pub fn pre_load_gltf_assets(mut commands: Commands, asset_server: Res<AssetServer>, environment: Res<CurrentEnvironment>) {
    let path = format!("environments/{}/{}", environment.environment.name, environment.area.name);
    let gltf_handle = asset_server.load::<Gltf>(path.as_str());

    commands.insert_resource(EffectSceneAssets(gltf_handle.clone()));
    info!("Pre Loading gltf for extras [{:?}]", path);
}

/// Processes a previously preloaded `.glb` area once it is fully loaded by Bevy's asset system.
/// This function extracts scenes from the `.glb` file and stores them in a resource for rendering.
///
/// Parameters:
/// - `commands`: Bevy's command buffer used to insert and remove resources.
/// - `gltf_assets`: The collection of all loaded GLTF assets.
/// - `next_state`: A mutable reference to the game's state, used to transition after loading.
/// - `waiting`: An optional resource that holds the handle for the area being loaded.
///
/// Behavior:
/// - Checks if the `.glb` file has finished loading.
/// - Retrieves up to three scenes (layers) from the `.glb` asset:
///   - **Layer 0**: Mandatory scene, causes a panic if missing.
///   - **Layer 1 & 2**: Optional layers; warnings are logged if they are missing.
/// - If additional scenes exist, they are considered potential battle scenes.
/// - Stores the loaded scenes in a `CurrentAreaScenes` resource.
/// - Removes the `WaitingForAreaAssets` resource.
/// - Transitions the game state to `GameState::EnvironmentPostLoad`.
///
/// Logging:
/// - Outputs how many scenes were found.
/// - Logs warnings if layers 1 or 2 are missing.
/// - Signals when environment loading is complete.
#[coverage(off)]
pub fn process_loaded_area(mut commands: Commands,
                           gltf_assets: Res<Assets<Gltf>>,
                           waiting: Option<Res<WaitingForAreaAssets>>,
                           mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(waiting) = waiting {
        if let Some(gltf) = gltf_assets.get(&waiting.0) {
            let mut map = HashMap::new();
            let found_scenes = gltf.scenes.len();
            info!("Found [{:?}] scenes", found_scenes);

            let layer_0 = gltf.scenes.get(0).cloned().expect("Scene 0 not found. This is Panic because we need minimum one scene!");
            let layer_1 = gltf.scenes.get(1).cloned();
            let layer_2 = gltf.scenes.get(2).cloned();

            if found_scenes > 4 {
                info!("Battle Scenes was found!");
                let mut count = 1;
                for (index, scene) in gltf.scenes.iter().enumerate() {
                    if index > 3 {
                        map.insert(format!("battle_{}", count), scene.clone());
                        count += 1;
                    }
                }
            }

            map.insert(String::from("layer_0"), layer_0.clone());
            if let Some(scene) = layer_1 {
                map.insert(String::from("layer_1"), scene.clone());
            } else {
                warn!("No Layer for Scene 1 found!");
            }

            if let Some(scene) = layer_2 {
                map.insert(String::from("layer_2"), scene.clone());
            } else {
                warn!("No Layer for Scene 2 found!");
            }

            commands.insert_resource(CurrentAreaScenes(map));
            commands.remove_resource::<WaitingForAreaAssets>();
            next_state.set(GameState::PostLoad);
            info!("Finished loading environments");
        }
    }
}

/// Spawns the loaded area assets into the game world.
///
/// This function retrieves the preloaded area scenes from the `CurrentAreaScenes` resource
/// and spawns them into the game world. The first and last layers include colliders,
/// while the second layer is purely visual. After spawning, the game state transitions to `GameState::InGame(InGameState::Main)`.
///
/// # Arguments
///
/// * `commands` - Used to spawn entities into the world.
/// * `current_area_scenes` - Hold the loaded area scenes.
/// * `next_state` - Used to transition to the next game state.
pub fn load_active_area(mut commands: Commands,
                        current_area_scenes: Res<CurrentAreaScenes>,
) {
    let first_layer = current_area_scenes.0.get(&String::from("layer_0")).cloned();
    let second_layer = current_area_scenes.0.get(&String::from("layer_1")).cloned();
    let last_layer = current_area_scenes.0.get(&String::from("layer_2")).cloned();

    if let Some(first_layer) = first_layer {
        commands.spawn(SceneRoot(first_layer.clone()))
            .insert(Name::new("Area First Layer"))
            .insert(EnvironmentScene)
            .insert(NoFrustumCulling)
            .insert(RigidBody::Fixed)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

    if let Some(second_layer) = second_layer {
        commands.spawn(SceneRoot(second_layer.clone()))
            .insert(Name::new("Area Second Layer"))
            .insert(NoFrustumCulling)
            .insert(EnvironmentScene);
    }

    if let Some(last_layer) = last_layer {
        commands.spawn(SceneRoot(last_layer.clone()))
            .insert(Name::new("Area Last Layer"))
            .insert(EnvironmentScene)
            .insert(RigidBody::Fixed)
            .insert(NoFrustumCulling)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

}

/// Loads active area lights from GLTF extra scene assets and spawns them into the world.
///
/// # Parameters
/// - `commands`: Commands for spawning entities.
/// - `next_state`: The next game state to transition to after loading lights.
/// - `gltf_assets`: GLTF asset resources.
/// - `gltf_nodes`: GLTF node resources containing extra metadata.
/// - `extra_scene_assets`: Optional extra scene assets that may contain light data.
pub fn load_active_area_lights(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    extra_scene_assets: Option<Res<EffectSceneAssets>>,
) {
    if let Some(layer_lights) = extra_scene_assets {
        if let Some(gltf) = gltf_assets.get(&layer_lights.0) {
            process_gltf_lights(&mut commands, &gltf, &gltf_nodes);
        }
    }
}

/// Processes GLTF nodes to extract light information and spawn them into the world.
///
/// # Parameters
/// - `commands`: Mutable reference to commands for spawning entities.
/// - `gltf`: Reference to the loaded GLTF asset.
/// - `gltf_nodes`: Reference to the GLTF node assets.
fn process_gltf_lights(
    commands: &mut Commands,
    gltf: &Gltf,
    gltf_nodes: &Assets<GltfNode>,
) {
    for node_handle in &gltf.nodes {
        if let Some(node) = gltf_nodes.get(node_handle) {
            if let Some(extras) = &node.extras {
                info!("Value: {:?}", &extras.value);
                if let Ok(parsed) = serde_json::from_str::<Value>(&extras.value) {
                    if let Some(bevy_json) = parsed.get("bevy_value").and_then(|v| v.as_str()) {
                        debug!("Json: {:?}", bevy_json);
                        if let Ok(light_data) = serde_json::from_str::<LightData>(bevy_json) {
                            spawn_light(commands, node, light_data);
                        }
                    }
                }
            }
        }
    }
}
/// Spawns a light entity based on the extracted light data.
///
/// # Parameters
/// - `commands`: Mutable reference to commands for spawning entities.
/// - `node`: Reference to the GLTF node containing transformation data.
/// - `light_data`: The extracted light data to configure the light entity.
fn spawn_light(commands: &mut Commands, node: &GltfNode, light_data: LightData) {
    debug!("Spawning light: {:?}", light_data);
    let light = match light_data.name.as_str() {
        "point" => LightType::Point(PointLight {
            intensity: light_data.intensity.unwrap_or(1000.0),
            range: light_data.range.unwrap_or(2.0),
            color: Color::srgb(light_data.color[0], light_data.color[1], light_data.color[2]),
            radius: light_data.radius.unwrap_or(1.5),
            shadows_enabled: light_data.shadows.unwrap_or(true),
            ..Default::default()
        }),
        "spot" => LightType::Spot(SpotLight {
            intensity: light_data.intensity.unwrap_or(1000.0),
            color: Color::srgb(light_data.color[0], light_data.color[1], light_data.color[2]),
            inner_angle: light_data.inner_cone.unwrap_or(PI / 4.0 * 0.85),
            outer_angle: light_data.outer_cone.unwrap_or(PI / 4.0),
            shadows_enabled: light_data.shadows.unwrap_or(true),
            radius: light_data.radius.unwrap_or(1.0),
            range: light_data.range.unwrap_or(2.0),
            ..Default::default()
        }),
        _ => return,
    };

    let transform = Transform {
        translation: node.transform.translation,
        rotation: node.transform.rotation,
        scale: node.transform.scale,
    };
    match light {
        LightType::Point(point_light) => commands.spawn((point_light, transform)),
        LightType::Spot(spot_light) => commands.spawn((spot_light, transform)),
    };
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use bevy::asset::weak_handle;
    use bevy::gltf::{GltfNode, GltfPlugin};
    use bevy::log::LogPlugin;
    use bevy::prelude::*;
    use bevy::render::mesh::MeshPlugin;
    use bevy::scene::ScenePlugin;
    use bevy_rapier3d::plugin::NoUserData;
    use bevy_rapier3d::prelude::{AsyncSceneCollider, RapierPhysicsPlugin};
    use game_system::app_state::GameState;
    use game_system::models::environment::{Area, CurrentAreaScenes, CurrentEnvironment, EffectSceneAssets, Environment, EnvironmentScene, EnvironmentState};
    use crate::environment::ready_handles::{load_active_area, load_active_area_lights, pre_load_gltf_assets};

    #[test]
    fn test_pre_load_gltf_assets() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default(), GltfPlugin::default()));
        let asset_server = app.world_mut().resource::<AssetServer>().clone();
        app.insert_resource(EffectSceneAssets(weak_handle!("7fd67c89-4467-4199-8c7d-51aa6dd25977")));

        let area = Area {
            index: 0,
            player_in_bound: false,
            name: "Area 1".to_string(),
            battle_scenes: Default::default(),
        };

        let environment = Environment {
            name: "Environment 1".to_string(),
            loaded: false,
            areas: vec![
                ("area1".to_string(), area.clone()),
            ].into_iter().collect(),
            state: EnvironmentState::Exploring,
        };

        app.insert_resource(CurrentEnvironment {
            environment,
            area,
        });

        app.add_systems(Startup, pre_load_gltf_assets);
        app.update();

        let effect_scene_assets = app.world().resource::<EffectSceneAssets>();

        let path = format!("environments/{}/{}", "Environment 1", "Area 1");
        let expected_handle = asset_server.load::<Gltf>(path.as_str());
        assert_eq!(effect_scene_assets.0, expected_handle);
    }

    #[test]
    fn test_load_active_area_spawns_entities() {
        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            GltfPlugin::default(),
            MeshPlugin,
            ScenePlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default()));

        // Dummy Handles
        let handle_0 = weak_handle!("05c72e8d-6b7f-4c44-b32b-04e96d33229a");
        let handle_1 = weak_handle!("34dca818-d90e-4f10-b823-ec1ee8aa77f0");
        let handle_2 = weak_handle!("5a2c1ed7-e345-4075-88bc-c5ab53bfb022");

        let mut scenes = HashMap::new();
        scenes.insert("layer_0".to_string(), handle_0.clone());
        scenes.insert("layer_1".to_string(), handle_1.clone());
        scenes.insert("layer_2".to_string(), handle_2.clone());

        app.insert_resource(CurrentAreaScenes(scenes));

        app.add_systems(Update, load_active_area);
        app.update();

        let world = app.world_mut();

        let mut layer_0_found = false;
        let mut layer_1_found = false;
        let mut layer_2_found = false;

        let mut query = world.query::<(Entity, &SceneRoot, &Name, &EnvironmentScene)>();
        for (entity, scene_root, name, _) in query.iter(world) {
            match name.as_str() {
                "Area First Layer" => {
                    assert_eq!(scene_root.0, handle_0);
                    layer_0_found = true;

                    let collider = world.get::<AsyncSceneCollider>(entity);
                    assert!(collider.is_some(), "Collider missing on Area First Layer");
                },
                "Area Second Layer" => {
                    assert_eq!(scene_root.0, handle_1);
                    layer_1_found = true;
                },
                "Area Last Layer" => {
                    assert_eq!(scene_root.0, handle_2);
                    layer_2_found = true;

                    let collider = world.get::<AsyncSceneCollider>(entity);
                    assert!(collider.is_some(), "Collider missing on Area Last Layer");
                },
                _ => {}
            }
        }

        assert!(layer_0_found, "Layer 0 not spawned");
        assert!(layer_1_found, "Layer 1 not spawned");
        assert!(layer_2_found, "Layer 2 not spawned");
    }

    #[test]
    fn test_load_active_area_lights() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, LogPlugin::default(), AssetPlugin::default(), GltfPlugin::default()));

        // Add fake game state resources
        app.insert_resource(NextState::<GameState>::default());

        // Dummy GLTF handle
        let gltf_handle = weak_handle!("1347c9b7-c46a-48e7-b7b8-023a354b7cac");

        // Insert EffectSceneAssets resource
        app.insert_resource(EffectSceneAssets(gltf_handle.clone()));

        // Create fake Gltf with node pointing to a light
        let mut gltf_assets = Assets::<Gltf>::default();
        let mut gltf_nodes = Assets::<GltfNode>::default();

        let _light_entity = Entity::from_raw(42);

        // Add dummy GltfNode with a light
        let node_handle_point = weak_handle!("8a893d29-c7ee-43cb-b0fa-4d28f2854a91");
        let node_handle_spot = weak_handle!("cfe68725-3dc7-44ab-b21e-8e04ce994fd3");
        gltf_nodes.insert(node_handle_point.clone().id(), GltfNode {
            index: 2,
            name: "Test Light 1".to_string(),
            mesh: None,
            skin: None,
            transform: Default::default(),
            is_animation_root: false,
            children: vec![],
            extras: Some(GltfExtras {
                value: "{\"bevy_value\":\"{ \\\"name\\\": \\\"point\\\", \\\"intensity\\\": 150000.0, \\\"range\\\": 10.0, \\\"radius\\\": 3.5 , \\\"color\\\": [ 0.7, 0.0, 0.8 ], \\\"shadows\\\": true }\"}"
                    .to_string(),
            }),
        });

        gltf_nodes.insert(node_handle_spot.clone().id(), GltfNode {
            index: 3,
            name: "Test Light 2".to_string(),
            mesh: None,
            skin: None,
            transform: Default::default(),
            is_animation_root: false,
            children: vec![],
            extras: Some(GltfExtras {
                value: "{\"bevy_value\":\"{ \\\"name\\\": \\\"spot\\\", \\\"intensity\\\": 1500000.0, \\\"range\\\": 45.0, \\\"radius\\\": 12.5 , \\\"color\\\": [ 1.0, 1.0, 1.0 ], \\\"shadows\\\": true, \\\"inner_cone\\\": 0.2, \\\"outer_cone\\\": 0.8 }\"}"
                    .to_string(),
            }),
        });

        // Add Gltf referencing this node
        gltf_assets.insert(gltf_handle.clone().id(), Gltf {
            scenes: vec![],
            named_scenes: Default::default(),
            meshes: vec![],
            named_meshes: Default::default(),
            materials: vec![],
            named_materials: Default::default(),
            nodes: vec![node_handle_point, node_handle_spot],
            named_nodes: Default::default(),
            skins: vec![],
            named_skins: Default::default(),
            default_scene: None,
            animations: vec![],
            named_animations: Default::default(),
            source: None,
        });

        app.insert_resource(gltf_assets);
        app.insert_resource(gltf_nodes);

        // Add system and run
        app.add_systems(Update, load_active_area_lights);
        app.update();
    }
}