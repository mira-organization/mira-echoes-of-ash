use bevy::prelude::*;
use bevy_extended_ui::html::HtmlFunctionRegistry;
use game_system::models::inventory::{InventoryOpen, InventoryState, Item};

pub struct InventoryController;

impl Plugin for InventoryController {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_functions);
    }
    
}

#[coverage(off)]
fn register_functions(mut functions: ResMut<HtmlFunctionRegistry>) {
    functions.click.insert("select_item".to_string(), select_item);
}

#[coverage(off)]
fn select_item(event: Trigger<Pointer<Click>>, mut commands: Commands) {
    let target = event.target();

    commands.queue(move |world: &mut World| {
        let item = {
            let mut query = world.query::<&Item>();
            query.get_mut(world, target).ok().cloned()
        };

        if let Some(item) = item {
            let mut needs_update = false;

            {
                let mut inventory_state = world.resource_mut::<InventoryState>();

                match inventory_state.selected_item.clone() {
                    Some(selected_item) if selected_item != item => {
                        inventory_state.selected_item = Some(item);
                        needs_update = true;
                    }
                    None => {
                        inventory_state.selected_item = Some(item);
                        needs_update = true;
                    }
                    _ => {}
                }
            }

            if needs_update {
                let mut inventory_open = world.resource_mut::<InventoryOpen>();
                inventory_open.updated = false;
                info!("Inventory updated");
            }
        }
    });
}