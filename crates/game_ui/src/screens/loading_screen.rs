use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use bevy_extended_ui::styling::convert::CssID;
use bevy_extended_ui::widgets::HtmlBody;
use game_system::app_state::GameState;

#[derive(Resource, Deref, DerefMut)]
struct LoadingScreenTimer(Timer);

pub struct LoadingScreen;

impl Plugin for LoadingScreen {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadGameAssets), display_loading_screen);
        app.add_systems(Update, hide_loading_screen.run_if(in_state(GameState::InGame)));
    }
}

#[coverage(off)]
fn display_loading_screen(mut commands: Commands) {
    commands.spawn(HtmlSource(String::from("assets/html/loading_screen.html")));
    commands.insert_resource(LoadingScreenTimer(Timer::from_seconds(0.5, TimerMode::Once)));
}

#[coverage(off)]
fn hide_loading_screen(
    mut commands: Commands,
    mut timer: ResMut<LoadingScreenTimer>,
    time: Res<Time>,
    query: Query<(Entity, &CssID), With<HtmlBody>>,
) {
    timer.tick(time.delta());

    if timer.finished() {
        for (entity, id) in query.iter() {
            if id.0.as_str() == "loading_screen" {
                commands.entity(entity).despawn();
            }
        }
    }
}