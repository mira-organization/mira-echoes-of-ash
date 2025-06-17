use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use bevy_extended_ui::styling::convert::CssID;
use bevy_rapier3d::prelude::DebugRenderContext;
use game_system::app_state::GameState;
use game_system::models::logic::WorldInspectorState;

pub struct HudScreen;

#[derive(Component)]
struct ObserverRegistered;

impl Plugin for HudScreen {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), generate_hud);
        app.add_systems(Update, (control_inspector_state, control_rapier_debug_state).run_if(in_state(GameState::InGame)));
    }
}

#[coverage(off)]
fn generate_hud(mut commands: Commands) {
    commands.spawn(HtmlSource(String::from("assets/html/hud.html")));
}

#[coverage(off)]
fn control_inspector_state(mut commands: Commands, query: Query<(Entity, &CssID), Without<ObserverRegistered>>) {
    for (entity, id) in query.iter() {
        if id.0.eq("inspector") {
            commands.entity(entity)                
                .insert(ObserverRegistered)
                .observe(|_: Trigger<Pointer<Click>>, mut inspector_state: ResMut<WorldInspectorState>| {
                    inspector_state.0 = !inspector_state.0;
            });
        }
    }
}

#[coverage(off)]
fn control_rapier_debug_state(mut commands: Commands, query: Query<(Entity, &CssID), Without<ObserverRegistered>>) {
    for (entity, id) in query.iter() {
        if id.0.eq("rapier-grid") {
            commands.entity(entity)
                .insert(ObserverRegistered)
                .observe(|_: Trigger<Pointer<Click>>, mut debug_context: ResMut<DebugRenderContext>,| {
                    debug_context.enabled = !debug_context.enabled
                });
        }
    }
}