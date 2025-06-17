use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use game_system::app_state::GameState;

pub struct HudScreen;

impl Plugin for HudScreen {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), generate_hud);
    }
}

#[coverage(off)]
fn generate_hud(mut commands: Commands) {
    commands.spawn(HtmlSource(String::from("assets/html/hud.html")));
}