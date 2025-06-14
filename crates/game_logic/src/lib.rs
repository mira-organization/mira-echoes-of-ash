#![feature(coverage_attribute)]

pub mod camera;

use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmospherePlugin;
use game_system::app_state::GameState;
use game_system::models::logic::WorldPlayer;
use game_system::models::party::CharacterPartyInfo;
use game_system::save_info::LoadedAssets;
use crate::camera::GameCameraPlugin;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin);
        app.add_plugins(GameCameraPlugin);
        app.add_systems(OnEnter(GameState::InGame), spawn_character);
    }
}

#[coverage(off)]
fn spawn_character(mut commands: Commands, party: Res<CharacterPartyInfo>, assets: Res<LoadedAssets>) {
    let character = party.active.clone();
    
    let mut  scene = Default::default();
    for (key, handles) in assets.characters.clone() {
        if key.eq(&character.name.clone()) {
            scene = handles;
        }
    }
    
    commands.spawn((
        Name::new("Test Player"),
        character.clone(),
        SceneRoot(scene),
        Transform::default(),
        WorldPlayer::default()
    ));
}