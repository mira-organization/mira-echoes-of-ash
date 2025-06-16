mod animation;
mod character;

use bevy::prelude::*;
use crate::player::animation::PlayerAnimationPlugin;
use crate::player::character::PlayerCharacterPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerAnimationPlugin, PlayerCharacterPlugin));
    }
}

/*#[coverage(off)]
fn entry_spawn_player(
    mut commands: Commands,
    party: Res<CharacterPartyInfo>,
    assets: Res<LoadedAssets>,
) {
    let character = party.active.clone();

    let mut scene = Default::default();
    for (key, handles) in assets.characters.clone() {
        if key.eq(&character.name.clone()) {
            scene = handles;
        }
    }

    commands.spawn((
        character.clone(),
        SceneRoot(scene),
        WorldPlayerBundle {
            transform: Transform::from_xyz(40.0, 13.0, 40.0),
            world_player: WorldPlayer {
                displayed_character: character.clone(),
                ..default()
            },
            ..default()
        }
    ));
}*/