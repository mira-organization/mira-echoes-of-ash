#![feature(coverage_attribute)]

#![feature(const_vec_string_slice)]
mod environment;
mod objects;
mod npc;

use bevy::prelude::*;
use crate::environment::EnvironmentPlugin;
use crate::npc::GameNpcPlugin;
use crate::objects::GameObjectsPlugin;

pub struct GameEnvironmentPlugin;

#[coverage(off)]
impl Plugin for GameEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EnvironmentPlugin, 
            GameObjectsPlugin,
            GameNpcPlugin
        ));
    }
}