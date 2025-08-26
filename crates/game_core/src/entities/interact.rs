#![coverage(off)]

use bevy::prelude::*;

/// Trait for sensor components that target another entity
pub trait SensorTarget {
    fn target_entity(&self) -> Entity;
}

/// Trait for resource that stores an optional nearby entity
pub trait NearbyTarget {
    fn set(&mut self, value: Option<Entity>);
    fn get(&self) -> Option<Entity>;
}