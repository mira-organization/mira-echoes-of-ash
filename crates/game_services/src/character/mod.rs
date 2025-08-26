mod switch;
mod camera_logic;
mod movement;

use bevy::prelude::*;
use crate::character::camera_logic::CameraLogic;
use crate::character::movement::MovementLogic;
use crate::character::switch::CharacterSwitchLogic;

pub struct CharacterService;

impl Plugin for CharacterService {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((CharacterSwitchLogic, MovementLogic, CameraLogic));
    }
}