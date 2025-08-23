use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use game_core::config::client_conf::GameConfig;
use game_core::{GameCorePlugin, WorldInspectorState};
use game_core::key_converter::convert;
use game_network::GameNetworkPlugin;
use game_services::GameServicesPlugin;
use game_ui::GameUiPlugin;
use game_world::GameWorldPlugin;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin {
            enabled: false,
            ..default()
        });
        app.add_plugins((
            GameCorePlugin,
            GameNetworkPlugin,
            GameServicesPlugin,
            GameWorldPlugin,
            GameUiPlugin
        ));
        
        app.add_systems(Update, (toggle_rapier_3d_grid, toggle_world_inspector));
    }
}

fn toggle_rapier_3d_grid(
    mut debug_context: ResMut<DebugRenderContext>,
    keyboard: ResMut<ButtonInput<KeyCode>>,
    game_config: Res<GameConfig>
) {
    let key = convert(game_config.input.rapier_debug.as_str())
        .expect("Invalid key for rapier debug grid");
    if keyboard.just_pressed(key) {
        debug_context.enabled = !debug_context.enabled;
    }
}

fn toggle_world_inspector(
    mut debug_context: ResMut<WorldInspectorState>,
    keyboard: ResMut<ButtonInput<KeyCode>>,
    game_config: Res<GameConfig>
) {
    let key = convert(game_config.input.world_inspector.as_str())
        .expect("Invalid key for world inspector");
    if keyboard.just_pressed(key) {
        debug_context.0 = !debug_context.0;
    }
}