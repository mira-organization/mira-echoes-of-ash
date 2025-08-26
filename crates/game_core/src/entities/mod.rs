use bevy::prelude::*;
use crate::entities::character::{ChangeCharacter, Character, CharacterPartyInfo, CurrentWorldCharacter, DotEffects};
use crate::entities::item::Item;

pub mod character;
pub mod item;
pub mod non_player;
pub mod player;
pub mod animation;
pub mod camera;
pub mod interact;

pub struct EntitiesModule;

impl Plugin for EntitiesModule {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterPartyInfo>();
        app.init_resource::<ChangeCharacter>();
        app.init_resource::<CurrentWorldCharacter>();
        app.register_type::<Character>();
        app.register_type::<DotEffects>();
        app.register_type::<Item>();
    }
}