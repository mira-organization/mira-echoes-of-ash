use bevy::prelude::*;
use bevy_rapier3d::prelude::{DebugRenderContext, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use game_audio::GameAudioPlugin;
use game_environment::GameEnvironmentPlugin;
use game_load::GameLoadPlugin;
use game_logic::GameLogicPlugin;
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
            GameLoadPlugin,
            GameUiPlugin,
            GameLogicPlugin,
            GameEnvironmentPlugin,
            GameAudioPlugin
        ));

        app.add_systems(Update, (toggle_debug_system, toggle_world_inspector_interface_system));
    }
}

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