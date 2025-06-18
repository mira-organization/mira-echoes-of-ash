use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use bevy_extended_ui::styling::convert::CssID;
use bevy_extended_ui::widgets::HtmlBody;
use game_system::app_state::GameState;
#[derive(Component)]
struct ObserverRegistered;

pub struct AccountScreen;

impl Plugin for AccountScreen {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AccountScreen), display_account_screen);
        app.add_systems(Update, control_debug_login.run_if(in_state(GameState::AccountScreen)));
        app.add_systems(OnExit(GameState::AccountScreen), hide_account_screen);
    }
}

#[coverage(off)]
fn display_account_screen(mut commands: Commands) {
    commands.spawn(HtmlSource(String::from("assets/html/account.html")));
}

#[coverage(off)]
fn control_debug_login(mut commands: Commands, query: Query<(Entity, &CssID), Without<ObserverRegistered>>) {
    for (entity, id) in query.iter() {
        if id.0.eq("debug-login") {
            commands.entity(entity)
                .insert(ObserverRegistered)
                .observe(|_: Trigger<Pointer<Click>>, mut next_game_state: ResMut<NextState<GameState>>| {
                    next_game_state.set(GameState::Preload);
                });
        }
    }
}

#[coverage(off)]
fn hide_account_screen(
    mut commands: Commands,
    query: Query<(Entity, &CssID), With<HtmlBody>>,
) {
    for (entity, id) in query.iter() {
        if id.0.as_str() == "account_screen" {
            commands.entity(entity).despawn();
        }
    }
}