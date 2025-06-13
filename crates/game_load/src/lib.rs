#![feature(coverage_attribute)]

use bevy::prelude::*;

pub struct GameLoadPlugin;

impl Plugin for GameLoadPlugin {
    #[coverage(off)]
    fn build(&self, _app: &mut App) {}
}