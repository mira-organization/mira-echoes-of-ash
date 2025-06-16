use bevy::prelude::*;

/// The `AnimatedPlayer` component is used to mark an entity as an animated player character.
///
/// This component is used for entities that represent the player in the game and have animations associated with them.
/// It does not carry any data but is used to identify player entities for animation purposes, such as handling character movement or combat animations.
#[derive(Component, Debug, Clone)]
pub struct AnimatedPlayer;

/// The `AnimatedMob` component is used to mark an entity as an animated mob (e.g., enemy or NPC).
///
/// This component is used for entities that represent mobs or non-player characters (NPCs) that have animations.
/// It helps to distinguish these entities from others in the game world and can be used to control their animation behavior, such as attack or movement animations.
#[derive(Component, Debug, Clone)]
pub struct AnimatedMob;