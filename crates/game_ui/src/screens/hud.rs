use bevy::prelude::*;
use bevy_extended_ui::html::HtmlSource;
use bevy_extended_ui::styling::convert::CssID;
use bevy_extended_ui::styling::paint::Colored;
use bevy_extended_ui::styling::system::WidgetStyle;
use bevy_extended_ui::widgets::Paragraph;
use bevy_rapier3d::prelude::DebugRenderContext;
use game_system::app_state::GameState;
use game_system::models::logic::WorldInspectorState;
use game_system::save_info::PingData;

pub struct HudScreen;

#[derive(Component)]
struct ObserverRegistered;

impl Plugin for HudScreen {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), generate_hud);
        app.add_systems(Update, (control_inspector_state, control_rapier_debug_state, update_ping).run_if(in_state(GameState::InGame)));
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

#[coverage(off)]
fn update_ping(
    mut query: Query<(&CssID, &mut WidgetStyle, &mut Paragraph), With<CssID>>,
    ping_res: Res<PingData>,
) {
    for (id, mut wid_style, mut p) in query.iter_mut() {
        if id.0 == "ping" {
            if let Some(rtt) = ping_res.last_rtt {
                let ms = rtt.as_millis();
                p.text = format!("{}ms", ms);

                let color: Color = if ms < 60 {
                    Colored::LIGHT_GREEN
                } else if ms < 250 {
                    Colored::ORANGE
                } else {
                    Colored::RED
                };

                for (_state, styles) in wid_style.styles.iter_mut() {
                    styles.color = Some(color);
                }

                if let Some(active) = wid_style.active_style.as_mut() {
                    active.color = Some(color);
                }
            }
        }
    }
}