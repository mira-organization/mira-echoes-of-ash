use bevy::prelude::*;
use bevy_extended_ui::registry::UiRegistry;
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::ui::{UiType, OpenUI};
use game_system::utils::convert;

pub struct MenuSystem;

impl Plugin for MenuSystem {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, open_menu.run_if(in_state(GameState::InGame)));
        app.add_systems(Update, close_settings_menu.run_if(in_state(GameState::InGame)));
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
        if open_ui.0.eq(&UiType::None) {
            ui_registry.use_ui("pause_menu_screen");
            open_ui.0 = UiType::Pause;
        } else if open_ui.0.eq(&UiType::Pause) {
            ui_registry.use_ui("hud");
            open_ui.0 = UiType::None;
        }
    }
}

#[coverage(off)]
fn close_settings_menu(
    ui_registry: Res<UiRegistry>,
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
) {
    let esc = convert(general_config.input_config.menu_key.as_str())
        .expect("Fetch key for (pause / esc) was failed!");
    
    if let Some(current) = ui_registry.current.clone() {
        if current.eq(&"settings_screen") && keyboard.just_pressed(esc) {
            general_config.save_all();
            debug!("Saved!");
        }
    }
}