use bevy::prelude::*;
use game_ui::GameUiPlugin;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(GameUiPlugin);
    }
}