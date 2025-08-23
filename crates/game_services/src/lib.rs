#![feature(coverage_attribute)]

mod loading;
mod character;
mod entities;

use bevy_atmosphere::prelude::AtmospherePlugin;
use bevy::prelude::*;
use crate::character::CharacterService;
use crate::entities::EntitiesService;
use crate::loading::LoadingService;

pub struct GameServicesPlugin;

impl Plugin for GameServicesPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin);
        app.add_plugins((LoadingService, CharacterService, EntitiesService));
    }
}