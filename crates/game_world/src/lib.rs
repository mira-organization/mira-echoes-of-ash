#![feature(coverage_attribute)]

mod environment;

use bevy::prelude::*;
use crate::environment::EnvironmentPlugin;

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(EnvironmentPlugin);
    }
}