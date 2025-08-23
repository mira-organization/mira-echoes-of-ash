mod splash;
mod account;
mod loading;
mod hud;

use bevy::prelude::*;
use crate::screens::account::AccountScreen;
use crate::screens::hud::HudScreen;
use crate::screens::loading::LoadingScreen;
use crate::screens::splash::SplashScreen;

pub struct ScreenManager;

impl Plugin for ScreenManager {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((SplashScreen, AccountScreen, LoadingScreen, HudScreen));
    }
}