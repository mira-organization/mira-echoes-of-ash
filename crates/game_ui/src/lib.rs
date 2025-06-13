#![feature(coverage_attribute)]

mod screens;

use bevy::prelude::*;
use bevy_extended_ui::ExtendedUiPlugin;
use crate::screens::SplashScreen;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtendedUiPlugin);
        app.add_plugins(SplashScreen);
    }
}