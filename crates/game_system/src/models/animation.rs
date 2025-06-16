use bevy::prelude::*;

/// Represents a collection of animations and their associated animation graph for an entity.
///
/// This component is used to define and manage animations for entities, such as enemies or characters,
/// by linking a series of animation nodes and an animation graph that dictates how these animations
/// transition and interact with one another.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Animations {
    /// A list of animation node indices that represent individual animations
    /// or states within the animation graph.
    ///
    /// These nodes can correspond to specific animation clips or poses, which are
    /// dynamically accessed and updated during gameplay.
    pub animations: Vec<AnimationNodeIndex>,

    /// A handle to the animation graph resource.
    ///
    /// The animation graph defines how animations transition between different states,
    /// such as walking, running, or attacking. This graph is typically loaded as an external
    /// asset and used by the entity to determine its current animation state.
    pub graph: Handle<AnimationGraph>,
}

/// The `AnimatedPlayer` component is used to mark an entity as an animated player character.
///
/// This component is used for entities that represent the player in the game and have animations associated with them.
/// It does not carry any data but is used to identify player entities for animation purposes, such as handling character movement or combat animations.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AnimatedPlayer;

/// The `AnimatedMob` component is used to mark an entity as an animated mob (e.g., enemy or NPC).
///
/// This component is used for entities that represent mobs or non-player characters (NPCs) that have animations.
/// It helps to distinguish these entities from others in the game world and can be used to control their animation behavior, such as attack or movement animations.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AnimatedMob;