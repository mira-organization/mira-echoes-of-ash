use bevy::prelude::*;
use bevy_extended_ui::registry::UiRegistry;
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::ui::{KnownUi, OpenUI};
use game_system::utils::convert;

pub struct MenuSystem;

impl Plugin for MenuSystem {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, open_menu.run_if(in_state(GameState::InGame)));
    }
}

#[coverage(off)]
fn open_menu(
    mut ui_registry: ResMut<UiRegistry>,
    mut open_ui: ResMut<OpenUI>,
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
) {
    let pause = convert(general_config.input_config.menu_key.as_str())
        .expect("Fetch key for (pause / esc) was failed!");
    if keyboard.just_pressed(pause) {
        if open_ui.0.eq(&KnownUi::None) {
            ui_registry.use_ui("pause_menu_screen");
            open_ui.0 = KnownUi::Pause;
        } else if open_ui.0.eq(&KnownUi::Pause) {
            ui_registry.use_ui("hud");
            open_ui.0 = KnownUi::None;
        }
    }
}