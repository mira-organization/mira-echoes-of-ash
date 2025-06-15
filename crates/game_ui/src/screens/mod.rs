pub mod splashscreen;
mod loading_screen;

use bevy::prelude::*;
use crate::screens::loading_screen::LoadingScreen;
use crate::screens::splashscreen::SplashScreen;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((SplashScreen, LoadingScreen));
    }
}