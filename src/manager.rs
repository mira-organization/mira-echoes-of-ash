use bevy::prelude::*;
use bevy_rapier3d::prelude::{DebugRenderContext, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use game_audio::GameAudioPlugin;
use game_environment::GameEnvironmentPlugin;
use game_load::GameLoadPlugin;
use game_logic::GameLogicPlugin;
use game_network::GameNetworkPlugin;
use game_system::config::ConfigService;
use game_system::GameSystemPlugin;
use game_system::models::logic::WorldInspectorState;
use game_system::utils::convert;
use game_ui::GameUiPlugin;

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
            GameSystemPlugin,
            GameNetworkPlugin,
            GameLoadPlugin,
            GameUiPlugin,
            GameLogicPlugin,
            GameEnvironmentPlugin,
            GameAudioPlugin
        ));

        app.add_systems(Update, (toggle_debug_system, toggle_world_inspector_interface_system));
    }
}

/// Toggles the debug rendering system on or off when the corresponding key is pressed.
///
/// The key binding is loaded from the general configuration under `input_config.debug_change`.
/// When pressed, this system inverts the `enabled` state of the `DebugRenderContext`.
///
/// # Panics
/// Panics if the debug toggle key defined in the configuration cannot be parsed.
///
/// # Parameters
/// - `debug_context`: A mutable resource controlling debug rendering.
/// - `keyboard`: Provides keyboard input state.
/// - `general_config`: Contains the input configuration for key bindings.
#[coverage(off)]
pub fn toggle_debug_system(
    mut debug_context: ResMut<DebugRenderContext>,
    keyboard: ResMut<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
) {
    let key = convert(general_config.input_config.debug_change.as_str())
        .expect("Fetch key for (debug change) was failed!");
    if keyboard.just_pressed(key) {
        debug_context.enabled = !debug_context.enabled
    }
}

/// Toggles the visibility of the World Inspector UI when the corresponding key is pressed.
///
/// The key binding is loaded from the general configuration under `input_config.world_inspector_ui`.
/// This system toggles the `WorldInspectorState` resource to show or hide the inspector interface.
///
/// # Panics
/// Panics if the world inspector toggle key defined in the configuration cannot be parsed.
///
/// # Parameters
/// - `keyboard`: Provides keyboard input state.
/// - `general_config`: Contains the input configuration for key bindings.
/// - `world_inspector_state`: A mutable resource indicating whether the world inspector is active.
#[coverage(off)]
pub fn toggle_world_inspector_interface_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
    mut world_inspector_state: ResMut<WorldInspectorState>,
) {
    let key = convert(general_config.input_config.world_inspector_ui.as_str())
        .expect("Fetch key for (world inspector ui) was failed!");

    if keyboard.just_pressed(key) {
        world_inspector_state.0 = !world_inspector_state.0;
    }
}