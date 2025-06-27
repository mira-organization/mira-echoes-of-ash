use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use bevy_extended_ui::html::{HtmlEventBindings, HtmlSource};
use bevy_extended_ui::registry::UiRegistry;
use bevy_extended_ui::styling::convert::{CssClass, CssID, CssSource};
use bevy_extended_ui::widgets::{Div, Headline, Img, Paragraph};
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::inventory::{InventoryOpen, InventoryState, Item};
use game_system::models::ui::{KnownUi, OpenUI};
use game_system::save_info::SaveInfo;
use game_system::utils::convert;

pub struct InventorySystem;

impl Plugin for InventorySystem {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), load_up_inventory);
        app.add_systems(Update, open_inventory.run_if(in_state(GameState::InGame)));
        app.add_systems(Update, update_inventory
            .run_if(in_state(GameState::InGame))
            .run_if(inventory_ui_ready));
    }
}

/// Loads the inventory UI layout from an HTML file and registers it with the UI registry.
///
/// This function should be called once (e.g., during initialization or state setup)
/// to make the `"inventory"` UI available.
///
/// # Parameters
/// - `ui_registry`: A mutable reference to the [`UiRegistry`] for managing UI definitions.
#[coverage(off)]
fn load_up_inventory(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.add("inventory".to_string(), HtmlSource::from_file_path("assets/html/game/inventory.html"));
}

/// Opens or closes the inventory UI based on player key input.
///
/// Toggles between `"inventory"` and `"hud"` UIs depending on whether the
/// inventory toggles key or escape key is pressed. Maintains the state via
/// [`InventoryOpen`].
///
/// # Parameters
/// - `ui_registry`: The current [`UiRegistry`] resource.
/// - `inventory_open`: Tracks whether the inventory is open and if it was updated.
/// - `keyboard`: The input state for key events.
/// - `general_config`: Configuration resource holding keybindings.
#[coverage(off)]
fn open_inventory(
    mut ui_registry: ResMut<UiRegistry>, 
    mut inventory_open: ResMut<InventoryOpen>,
    mut open_ui: ResMut<OpenUI>,
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
) {
    let button = convert(general_config.input_config.open_inventory.as_str())
        .expect("Fetch key for (open inventory) was failed!");
    let esc = convert(general_config.input_config.menu_key.as_str())
        .expect("Fetch key for (close) was failed!");
    
    if keyboard.just_pressed(button) {
        if let Some(active) = ui_registry.current.clone() {
            if active.eq(&"inventory") && inventory_open.open {
                ui_registry.use_ui("hud");
                inventory_open.open = false;
                inventory_open.updated = false;
                open_ui.0 = KnownUi::None;
            } else {
                ui_registry.use_ui("inventory");
                inventory_open.open = true;
                open_ui.0 = KnownUi::Inventory;
            }
        } else {
            ui_registry.use_ui("inventory");
            inventory_open.open = true;
            open_ui.0 = KnownUi::Inventory;
        }
    } else if keyboard.just_pressed(esc) && !open_ui.0.eq(&KnownUi::None) {
        if inventory_open.open {
            inventory_open.open = false;
            inventory_open.updated = false;
            ui_registry.use_ui("hud");
            open_ui.0 = KnownUi::None;
        }
    }
}

/// Updates the inventory UI when newly opened or changed.
///
/// Adds missing items to the UI, updates descriptor fields like name, description, and icon,
/// and sets the visibility state for the selected item descriptor.
///
/// # Parameters
/// - `item_container_query`: Queries UI containers and elements with CSS IDs.
/// - `title_query`: Query for the title text of the selected item.
/// - `description_query`: Query for the description text of the selected item.
/// - `img_query`: Query for the item icon.
/// - `descriptor_query`: Query for visibility control of the item descriptor.
/// - `item_query`: Query to retrieve item components and their entities.
/// - `commands`: Commands used to spawn child elements.
/// - `save_info`: The current save state with available items.
/// - `inventory_open`: Resource tracking the open/updated state of the inventory.
/// - `inventory_state`: The currently selected tab and selected item.
#[coverage(off)]
fn update_inventory(
    item_container_query: Query<(Entity, Option<&Children>, &CssID, &CssSource)>,
    mut title_query: Query<&mut Headline>,
    mut description_query: Query<&mut Paragraph>,
    mut img_query: Query<&mut Img>,
    mut descriptor_query: Query<&mut Visibility>,
    mut item_query: Query<(&mut Item, Entity)>,
    mut commands: Commands,
    save_info: Res<SaveInfo>,
    mut inventory_open: ResMut<InventoryOpen>,
    inventory_state: Res<InventoryState>
) {
    if !inventory_open.updated {
        for (entity, children, id, source) in item_container_query.iter() {
            if id.0 == "item-container" {
                commands.entity(entity).with_children(|builder| {
                    for new_item in &save_info.items {
                        let mut found = false;

                        if let Some(children) = children {
                            for child in children.iter() {
                                if let Ok((existing_item, _)) = item_query.get_mut(child) {
                                    if existing_item.name == new_item.name {
                                        info!("Update item: {}", new_item.name);
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }

                        if !found {
                            info!("Add item: {}", new_item.name);
                            spawn_inventory_item(builder, new_item, source);
                        }
                    }
                });
            }
            
            if id.0 == "item-descriptor" {
                if let Ok(mut visibility) = descriptor_query.get_mut(entity) {
                    if inventory_state.selected_item.is_none() {
                        *visibility = Visibility::Hidden;
                    } else {
                        *visibility = Visibility::Visible;
                    }
                }
            }

            if id.0 == "item-name" {
                if let Some(item) = inventory_state.selected_item.clone() {
                    if let Ok(mut headline) = title_query.get_mut(entity) {
                        headline.text = item.display.clone();
                    }
                }
            }
            
            if id.0 == "item-icon" {
                if let Some(item) = inventory_state.selected_item.clone() {
                    if let Ok(mut img) = img_query.get_mut(entity) {
                        img.src = item.icon.clone();
                    }
                }
            }

            if id.0 == "item-description" {
                if let Some(item) = inventory_state.selected_item.clone() {
                    if let Ok(mut des) = description_query.get_mut(entity) {
                        des.text = item.description.clone();
                    }
                }
            }
        }

        inventory_open.updated = true;
    }
}

/// Spawns a new inventory item UI node with given styling and data bindings.
///
/// The item is wrapped in a `Div` node and styled according to its rarity.
/// Includes icon and value text.
///
/// # Parameters
/// - `builder`: Child entity builder used for spawning UI nodes.
/// - `item`: The item to display.
/// - `source`: The HTML/CSS source context for consistent styling.
#[coverage(off)]
fn spawn_inventory_item(builder: &mut RelatedSpawnerCommands<ChildOf>, item: &Item, source: &CssSource) {
    builder.spawn((
        item.clone(),
        Div::default(),
        Node::default(),
        CssClass(vec![
            "item-card".to_string(),
            format!("rarity-{}", item.rarity),
        ]),
        HtmlEventBindings {
            onclick: Some("select_item".to_string()),
            ..default()
        },
        source.clone(),
        children![
            (
                Div::default(),
                Node::default(),
                CssClass(vec!["item-display".to_string()]),
                source.clone(),
                children![
                    (
                        Img {
                           src: item.icon.clone(),
                           ..default()
                        },
                        Node::default(),
                        source.clone(),
                    )
                ]
            ),
            (
                Div::default(),
                Node::default(),
                CssClass(vec!["item-info".to_string()]),
                source.clone(),
                children![
                    (
                        Paragraph {
                            text: format!("{}", item.value.clone()),
                            ..default()
                        },
                        Node::default(),
                        source.clone(),
                    )
                ]
            )
        ]
    ));
}

/// Checks if the inventory UI is currently active and ready (i.e., the container exists).
///
/// This is useful to ensure the UI is ready before populating or modifying its contents.
///
/// # Parameters
/// - `ui_registry`: The current [`UiRegistry`] resource.
/// - `item_container_query`: Query to check for the presence of the `"item-container"` node.
///
/// # Returns
/// - `true` if the inventory UI is active and the container exists.
/// - `false` otherwise.
#[coverage(off)]
fn inventory_ui_ready(
    ui_registry: Res<UiRegistry>,
    item_container_query: Query<(&CssID, &CssSource)>,
) -> bool {
    if ui_registry.current.as_deref() != Some("inventory") {
        return false;
    }

    item_container_query.iter().any(|(id, _source)| id.0 == "item-container")
}

