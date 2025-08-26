mod loading_screen_controller;

use bevy::prelude::*;
use crate::controller::loading_screen_controller::LoadingScreenController;

pub struct ControllerManager;

impl Plugin for ControllerManager {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(
            LoadingScreenController
        );
    }
}