#![feature(coverage_attribute)]

mod environment;

use bevy::prelude::*;
use crate::environment::EnvironmentPlugin;

pub struct GameEnvironmentPlugin;

#[coverage(off)]
impl Plugin for GameEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnvironmentPlugin);
    }
}