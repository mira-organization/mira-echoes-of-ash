#![coverage(off)]

use std::collections::HashMap;
use bevy::prelude::*;
use crate::entities::item::Item;
use crate::json::entity::JsonEntity;

/// A Bevy resource that holds all entities loaded from JSON.
///
/// This resource wraps a `Vec<JsonEntity>` and is intended to be populated
/// by the `load_json_entities` system. By default, it starts out empty.
/// You can query or mutate this resource in your systems to access the
/// globally loaded JSON entities.
///
/// # Examples
///
/// ```ignore
/// // In your Bevy app setup:
/// app
///     .init_resource::<GlobalEntities>()
///     .add_system(load_json_entities);
///
/// // In another system:
/// fn use_entities(global_entities: Res<GlobalEntities>) {
///     for entity in global_entities.0.iter() {
///         println!("Entity ID: {}", entity.id);
///     }
/// }
/// ```
#[derive(Resource, Default, Clone, Debug)]
pub struct GlobalEntities(pub Vec<JsonEntity>);

/// A Bevy resource that holds the global registry of items.
///
/// `GlobalItems` maps each item’s unique identifier (a `String` key)
/// to its corresponding `Item` data. This allows systems to look up,
/// add, or modify items in the world by name.
///
/// Derived traits:
/// - `Default`: Initializes with an empty map.
/// - `Clone`: Enables cloning the entire registry.
/// - `Debug`: Provides a `Debug` representation for logging or inspection.
///
/// # Example
///
/// ```ignore
/// use std::collections::HashMap;
/// use my_crate::{GlobalItems, Item};
///
/// // Create a default, empty registry
/// let mut registry = GlobalItems::default();
///
/// // Insert a new item into the registry
/// registry.0.insert(
///     "health_potion".to_string(),
///     Item {
///         id: "health_potion".to_string(),
///         name: "Health Potion".to_string(),
///         // fill in other fields as needed...
///     },
/// );
///
/// // Retrieve and use an item
/// if let Some(potion) = registry.0.get("health_potion") {
///     println!("Found item: {} ({:?})", potion.name, potion);
/// } else {
///     println!("Item not found!");
/// }
/// ```
#[derive(Resource, Default, Clone, Debug)]
pub struct GlobalItems(
    /// A map from item identifier to the `Item` definition.
    pub HashMap<String, Item>,
);