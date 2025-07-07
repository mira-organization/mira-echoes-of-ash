use bevy::prelude::*;
use bevy_extended_ui::registry::UiRegistry;
use bevy_extended_ui::styling::convert::CssID;
use bevy_extended_ui::styling::paint::Colored;
use bevy_extended_ui::styling::system::WidgetStyle;
use bevy_extended_ui::widgets::{Headline, Img, Paragraph};
use bevy_rapier3d::prelude::DebugRenderContext;
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::inventory::{NearbyItem, WorldItem};
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
        app.add_systems(Update, update_item_dialog
            .run_if(in_state(GameState::InGame))
            .run_if(resource_changed::<NearbyItem>));
    }
}

/// Updates the item dialog UI based on the player's proximity to a world item.
///
/// <p>If a nearby item exists, the corresponding UI components (title, icon, collect text)
/// are updated with the item's information. Otherwise, the item dialog is hidden.</p>
///
/// # Parameters
/// - `hud_query`: A query of all UI HUD elements with their corresponding `CssID`.
/// - `nearby_item`: Resource that holds the entity of the item the player is near, if any.
/// - `world_items`: Query to access `WorldItem` components attached to item entities.
/// - `title_query`: Query for modifying the item's title text.
/// - `text_query`: Query for modifying the collect instruction text.
/// - `img_query`: Query for modifying the item's icon.
/// - `dialog_query`: Query for toggling the visibility of the dialog.
/// - `general_config`: Contains player input configuration and general settings.
///
/// # Behavior
/// <ul>
///   <li>If a nearby item is found, show the item dialog and populate its fields.</li>
///   <li>If no item is nearby or the item cannot be found, the item dialog is hidden.</li>
/// </ul>
#[coverage(off)]
fn update_item_dialog(
    hud_query: Query<(Entity, &CssID)>,
    nearby_item: Res<NearbyItem>,
    world_items: Query<&WorldItem>,
    mut title_query: Query<&mut Headline>,
    mut text_query: Query<&mut Paragraph>,
    mut img_query: Query<&mut Img>,
    mut dialog_query: Query<&mut Visibility>,
    general_config: Res<ConfigService>,
) {
    if let Some(near_entity) = nearby_item.0 {
        if let Ok(item) = world_items.get(near_entity) {
            update_item_dialog_ui(
                &hud_query,
                &mut title_query,
                &mut text_query,
                &mut img_query,
                &mut dialog_query,
                Some(item),
                &general_config,
            );
            return;
        }
    }

    update_item_dialog_ui(
        &hud_query,
        &mut title_query,
        &mut text_query,
        &mut img_query,
        &mut dialog_query,
        None,
        &general_config,
    );
}

/// Registers the HUD UI by referencing the `"hud"` UI layout
/// from the [`UiRegistry`].
///
/// This is typically called during startup or state transitions to
/// initialize and display the HUD.
///
/// # Parameters
/// - `ui_registry`: A mutable reference to the [`UiRegistry`] resource used to manage UI definitions.
#[coverage(off)]
fn generate_hud(
    mut ui_registry: ResMut<UiRegistry>,
    nearby_item: Res<NearbyItem>,
    hud_query: Query<(Entity, &CssID)>,
    mut title_query: Query<&mut Headline>,
    mut text_query: Query<&mut Paragraph>,
    mut img_query: Query<&mut Img>,
    mut dialog_query: Query<&mut Visibility>,
    general_config: Res<ConfigService>,
    world_items: Query<&WorldItem>,
) {
    ui_registry.use_ui("hud");
    if let Some(near_entity) = nearby_item.0 {
        if let Ok(item) = world_items.get(near_entity) {
            update_item_dialog_ui(
                &hud_query,
                &mut title_query,
                &mut text_query,
                &mut img_query,
                &mut dialog_query,
                Some(item),
                &general_config,
            );
        }
    }
}

/// Controls the visibility of the [`WorldInspector`] debug panel
/// based on clicks on the UI element with the CSS ID `"inspector"`.
///
/// Attaches an observer to the element to toggle the [`WorldInspectorState`] boolean
/// when clicked. This allows enabling/disabling the in-game inspector.
///
/// # Parameters
/// - `commands`: A [`Commands`] object to modify entity components.
/// - `query`: A query to find entities with a `CssID` of `"inspector"` that have not been registered yet.
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

/// Controls the visibility of Rapier's debug render grid.
///
/// Binds a click observer to the UI element with CSS ID `"rapier-grid"`,
/// and toggles the visibility of physics debug rendering.
///
/// # Parameters
/// - `commands`: A [`Commands`] object to modify entity components.
/// - `query`: A query to find entities with a `CssID` of `"rapier-grid"` that have not been registered yet.
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

/// Updates the ping display in the HUD based on the last RTT value from [`PingData`].
///
/// Adjusts the text and color of the ping display UI element. Green for good, orange for moderate,
/// red for high latency.
///
/// # Parameters
/// - `query`: A query for UI elements with a [`CssID`] and style.
/// - `ping_res`: The current [`PingData`] resource containing the last RTT measurement.
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

#[coverage(off)]
#[allow(clippy::too_many_arguments)]
fn update_item_dialog_ui(
    hud_query: &Query<(Entity, &CssID)>,
    title_query: &mut Query<&mut Headline>,
    text_query: &mut Query<&mut Paragraph>,
    img_query: &mut Query<&mut Img>,
    dialog_query: &mut Query<&mut Visibility>,
    world_item_opt: Option<&WorldItem>,
    general_config: &ConfigService,
) {
    let interact_key_name = general_config.input_config.player_interact.to_string();

    for (entity, id) in hud_query.iter() {
        match id.0.as_str() {
            "item-dialog" => {
                if let Ok(mut visibility) = dialog_query.get_mut(entity) {
                    *visibility = if world_item_opt.is_some() {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };
                }
            }
            "dia-title" => {
                if let Some(item) = world_item_opt {
                    if let Ok(mut headline) = title_query.get_mut(entity) {
                        headline.text = item.item.display.clone();
                    }
                }
            }
            "dia-icon" => {
                if let Some(item) = world_item_opt {
                    if let Ok(mut img) = img_query.get_mut(entity) {
                        img.src = item.item.icon.clone();
                    }
                }
            }
            "collect-text" => {
                if let Some(_) = world_item_opt {
                    if let Ok(mut col_text) = text_query.get_mut(entity) {
                        col_text.text = format!("Collect [ {} ]", interact_key_name);
                    }
                }
            }
            _ => {}
        }
    }
}