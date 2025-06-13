#![feature(coverage_attribute)]

mod service;

use bevy::prelude::*;
use crate::service::ServicePlugin;

pub struct GameLoadPlugin;

impl Plugin for GameLoadPlugin {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(ServicePlugin);
    }
}