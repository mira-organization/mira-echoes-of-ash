pub mod environment;
pub mod non_players;
pub mod light;

use bevy::prelude::*;
use crate::world::environment::EnvironmentListResource;

pub struct WorldModule;

impl Plugin for WorldModule {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_resource::<EnvironmentListResource>();

    }
}