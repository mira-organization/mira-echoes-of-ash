use bevy::prelude::*;
use bevy_extended_ui::registry::UiRegistry;
use bevy_extended_ui::styling::convert::CssID;
use bevy_extended_ui::styling::paint::Colored;
use bevy_extended_ui::styling::system::WidgetStyle;
use bevy_extended_ui::widgets::{Headline, Img, Paragraph};
use bevy_rapier3d::prelude::DebugRenderContext;
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::dialog::DialogData;
use game_system::models::inventory::{NearbyItem, WorldItem};
use game_system::models::logic::WorldInspectorState;
use game_system::models::npcs::{NearbyNpc, NpcData};
use game_system::save_info::PingData;

pub struct HudScreen;

#[derive(Component)]
struct ObserverRegistered;

impl Plugin for HudScreen {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), generate_hud);
        app.add_systems(Update, (control_inspector_state, control_rapier_debug_state, update_ping).run_if(in_state(GameState::InGame)));
        app.add_systems(Update, (
            update_item_dialog
                .run_if(in_state(GameState::InGame))
                .run_if(resource_changed::<NearbyItem>),
            update_npc_dialog
                .run_if(in_state(GameState::InGame))
                .run_if(resource_changed::<NearbyNpc>)
        ));
    }
}

/// Updates the item dialog UI based on the currently nearby item.
///
/// Queries the HUD elements, finds the nearby item entity, and if present,
/// updates the dialog UI by calling `update_dialog_ui` with the item data.
///
/// # Parameters
/// * `hud_query` - Query of all HUD elements with their CSS IDs.
/// * `nearby_item` - Resource holding the currently nearby item entity.
/// * `world_items` - Query to get `WorldItem` components by entity.
/// * `title_query` - Mutable query to update the headline component.
/// * `text_query` - Mutable query to update the paragraph component.
/// * `img_query` - Mutable query to update the image component.
/// * `dialog_query` - Mutable query to update the dialog visibility.
/// * `general_config` - Resource providing configuration such as input keys.
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
    let item_opt = nearby_item.0.and_then(|e| world_items.get(e).ok());
    update_dialog_ui::<WorldItem>(
        &hud_query,
        &mut title_query,
        &mut text_query,
        Some(&mut img_query),
        &mut dialog_query,
        item_opt,
        &general_config,
    );
}

/// Updates the NPC dialog UI based on the currently nearby NPC.
///
/// Queries the HUD elements, finds the nearby NPC entity, and if present,
/// updates the dialog UI by calling `update_dialog_ui` with the NPC data.
///
/// # Parameters
/// * `hud_query` - Query of all HUD elements with their CSS IDs.
/// * `nearby_npc` - Resource holding the currently nearby NPC entity.
/// * `npc_query` - Query to get `NpcData` components by entity.
/// * `title_query` - Mutable query to update the headline component.
/// * `text_query` - Mutable query to update the paragraph component.
/// * `dialog_query` - Mutable query to update the dialog visibility.
/// * `general_config` - Resource providing configuration such as input keys.
#[coverage(off)]
fn update_npc_dialog(
    hud_query: Query<(Entity, &CssID)>,
    nearby_npc: Res<NearbyNpc>,
    npc_query: Query<&NpcData>,
    mut title_query: Query<&mut Headline>,
    mut text_query: Query<&mut Paragraph>,
    mut dialog_query: Query<&mut Visibility>,
    general_config: Res<ConfigService>,
) {
    let npc_opt = nearby_npc.0.and_then(|e| npc_query.get(e).ok());
    update_dialog_ui::<NpcData>(
        &hud_query,
        &mut title_query,
        &mut text_query,
        None,
        &mut dialog_query,
        npc_opt,
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

    let item_opt = nearby_item.0.and_then(|e| world_items.get(e).ok());
    update_dialog_ui::<WorldItem>(
        &hud_query,
        &mut title_query,
        &mut text_query,
        Some(&mut img_query),
        &mut dialog_query,
        item_opt,
        &general_config,
    );
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

/// Updates dialog UI elements based on the provided dialog data.
///
/// This generic function updates multiple UI elements identified by their `CssID`, including
/// - The visibility of the dialog box
/// - The title (headline)
/// - The main text (paragraph)
/// - An optional icon image
///
/// The function supports any type that implements the `DialogData` trait. If `data_opt` is `Some`,
/// the dialog will be made visible and filled with dynamic content. If `data_opt` is `None`, it will
/// hide the dialog using the placeholder visibility ID.
///
/// # Type Parameters
/// * `T` - A type that implements the `DialogData` trait.
///
/// # Parameters
/// * `hud_query` - A query over all HUD elements, each tagged with a `CssID`.
/// * `title_query` - A mutable query used to update the headline (title) text component.
/// * `text_query` - A mutable query used to update the paragraph (main body) text component.
/// * `img_query` - An optional mutable query used to update an image component (e.g., character portrait).
/// * `dialog_query` - A mutable query used to show or hide the dialog UI element.
/// * `data_opt` - Optional dialog data. If `Some`, the UI is populated. If `None`, the dialog is hidden.
/// * `general_config` - Global configuration resource, used to retrieve input bindings like the interacted key.
///
/// # Behavior
/// * When `data_opt` is `Some`, matching UI elements are updated with values from `DialogData`.
/// * When `data_opt` is `None`, only the visibility element with the placeholder ID is hidden.
///
/// This will populate the NPC dialog UI with the NPC’s name, dialog text, and optional icon.
#[coverage(off)]
#[allow(clippy::too_many_arguments)]
fn update_dialog_ui<T: DialogData>(
    hud_query: &Query<(Entity, &CssID)>,
    title_query: &mut Query<&mut Headline>,
    text_query: &mut Query<&mut Paragraph>,
    mut img_query: Option<&mut Query<&mut Img>>,
    dialog_query: &mut Query<&mut Visibility>,
    data_opt: Option<&T>,
    general_config: &ConfigService,
) {
    let interact_key = general_config.input_config.player_interact.to_string();

    for (entity, id) in hud_query.iter() {

        if let Some(data) = data_opt {
            if id.0 == T::dialog_visible_id(data) {
                if let Ok(mut visibility) = dialog_query.get_mut(entity) {
                    *visibility = Visibility::Inherited;
                }
            }

            if id.0 == T::title_id(data) {
                if let Ok(mut headline) = title_query.get_mut(entity) {
                    headline.text = data.title_text();
                }
            }

            if id.0 == T::text_id(data) {
                if let Ok(mut paragraph) = text_query.get_mut(entity) {
                    paragraph.text = data.main_text(&interact_key);
                }
            }

            if let (Some(img_query), Some(icon_id)) = (img_query.as_mut(), data.icon_id()) {
                if id.0 == icon_id {
                    if let Some(icon) = data.icon() {
                        if let Ok(mut img) = img_query.get_mut(entity) {
                            img.src = Some(icon);
                        }
                    }
                }
            }
        } else {
            if id.0 == T::dialog_visible_id_placeholder() {
                if let Ok(mut visibility) = dialog_query.get_mut(entity) {
                    *visibility = Visibility::Hidden;
                }
            }
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