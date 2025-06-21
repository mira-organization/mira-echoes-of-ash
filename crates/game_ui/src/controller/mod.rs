mod loading_screen_controller;

use bevy::prelude::*;
use crate::controller::loading_screen_controller::LoadingScreenController;

pub struct UiControllerPlugin;

impl Plugin for UiControllerPlugin {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingScreenController);
    }
}