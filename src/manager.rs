use bevy::prelude::*;
use bevy_rapier3d::prelude::{DebugRenderContext, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use game_load::GameLoadPlugin;
use game_logic::GameLogicPlugin;
use game_system::config::ConfigService;
use game_system::GameSystemPlugin;
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
            GameLogicPlugin
        ));

        app.add_systems(Update, toggle_debug_system);
    }
}

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