#![feature(coverage_attribute)]

pub mod camera;
pub mod input;
pub mod player;

use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmospherePlugin;
use crate::camera::GameCameraPlugin;
use crate::input::GameInputPlugin;
use crate::player::PlayerPlugin;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin);
        app.add_plugins((GameCameraPlugin, PlayerPlugin, GameInputPlugin));
    }
}