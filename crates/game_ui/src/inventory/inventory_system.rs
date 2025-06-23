use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use bevy_extended_ui::registry::UiRegistry;
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::inventory::InventoryOpen;
use game_system::utils::convert;

pub struct InventorySystem;

impl Plugin for InventorySystem {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), load_up_inventory);
        app.add_systems(Update, open_inventory.run_if(in_state(GameState::InGame)));
    }
}

fn load_up_inventory(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.add("inventory".to_string(), HtmlSource::from_file_path("assets/html/game/inventory.html"));
}

fn open_inventory(
    mut ui_registry: ResMut<UiRegistry>, 
    mut inventory_open: ResMut<InventoryOpen>, 
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>
) {
    let button = convert(general_config.input_config.open_inventory.as_str())
        .expect("Fetch key for (open inventory) was failed!");
    
    if keyboard.just_pressed(button) {
        if let Some(active) = ui_registry.current.clone() {
            if active.eq(&"inventory") && inventory_open.0 {
                info!("Close");
                ui_registry.use_ui("hud");
                inventory_open.0 = false;
            } else {
                ui_registry.use_ui("inventory");
                inventory_open.0 = true;
            }
        } else {
            ui_registry.use_ui("inventory");
            inventory_open.0 = true;
        }
    }
}


