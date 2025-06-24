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

/// Registers inventory-related HTML event functions into the global function registry.
///
/// This function links the `"select_item"` `onclick` attribute in HTML to the
/// corresponding Rust callback [`select_item`] used for handling item selection.
///
/// # Parameters
/// - `functions`: The [`HtmlFunctionRegistry`] resource used to store event bindings.
#[coverage(off)]
fn register_functions(mut functions: ResMut<HtmlFunctionRegistry>) {
    functions.click.insert("select_item".to_string(), select_item);
}

/// Handles click events on inventory item UI elements to update the selected item.
///
/// When the user clicks on an item card in the inventory UI, this function sets the
/// clicked item as the newly selected item in [`InventoryState`]. If the selected item
/// has changed, the [`InventoryOpen`] resource is marked as `updated = false` to
/// trigger a UI refresh.
///
/// This function is queued to run inside a `World` context using [`Commands::queue`]
/// to safely access resources and components.
///
/// # Parameters
/// - `event`: The pointer click event triggered by an item UI element.
/// - `commands`: The [`Commands`] object for enqueuing world updates.
///
/// # Notes
/// - The target entity must contain an [`Item`] component to be selectable.
/// - The item is only updated if it differs from the previously selected one.
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