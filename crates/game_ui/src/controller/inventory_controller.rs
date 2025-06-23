use bevy::prelude::*;
use bevy_extended_ui::html::HtmlFunctionRegistry;
use bevy_extended_ui::UIWidgetState;

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
    commands.queue(move | world: &mut World | {
        let mut state_query = world.query::<&mut UIWidgetState>();
        if let Ok(mut state) = state_query.get_mut(world, target) {
            state.focused = !state.focused;
        }
    });
}