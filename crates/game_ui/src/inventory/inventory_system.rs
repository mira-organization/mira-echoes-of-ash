use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use bevy_extended_ui::html::{HtmlEventBindings, HtmlSource};
use bevy_extended_ui::registry::UiRegistry;
use bevy_extended_ui::styling::convert::{CssClass, CssID, CssSource};
use bevy_extended_ui::widgets::{Div, Headline, Img, Paragraph};
use game_system::app_state::GameState;
use game_system::config::ConfigService;
use game_system::models::inventory::{InventoryOpen, InventoryState, Item};
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

#[coverage(off)]
fn load_up_inventory(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.add("inventory".to_string(), HtmlSource::from_file_path("assets/html/game/inventory.html"));
}

#[coverage(off)]
fn open_inventory(
    mut ui_registry: ResMut<UiRegistry>, 
    mut inventory_open: ResMut<InventoryOpen>, 
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
) {
    let button = convert(general_config.input_config.open_inventory.as_str())
        .expect("Fetch key for (open inventory) was failed!");
    
    if keyboard.just_pressed(button) {
        if let Some(active) = ui_registry.current.clone() {
            if active.eq(&"inventory") && inventory_open.open {
                ui_registry.use_ui("hud");
                inventory_open.open = false;
                inventory_open.updated = false;
            } else {
                ui_registry.use_ui("inventory");
                inventory_open.open = true;
            }
        } else {
            ui_registry.use_ui("inventory");
            inventory_open.open = true;
        }
    }
}

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
                        headline.text = item.name.clone();
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

