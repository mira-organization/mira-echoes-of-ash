mod light_handler;
mod world_init;
mod world_build;

use bevy::prelude::*;
use crate::environment::light_handler::LightHandler;
use crate::environment::world_build::WorldBuildHandler;
use crate::environment::world_init::WorldInitHandler;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LightHandler,
            WorldInitHandler,
            WorldBuildHandler
        ));
    }
}