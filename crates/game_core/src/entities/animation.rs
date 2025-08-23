use bevy::prelude::*;

/// High-level role marker for an animatable entity.
///
/// Attach this to the **`AnimationPlayer` child** (together with [`AnimOwner`]) to
/// indicate what kind of logic should compose its animation intent.
/// - `Player`: driven by player input / `WorldPlayerState`
/// - `Npc`: driven by NPC AI state
/// - `Enemy`: driven by enemy AI state
/// - `Prop`: world objects (e.g., windmills, doors)
///
/// Composer systems map role-specific gameplay state → [`AnimIntent`].
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum AnimRole { Player, Npc, Enemy, Prop }

/// Links an `AnimationPlayer` child to its logical owner/root entity (the one
/// that holds gameplay state like `WorldPlayer`, `Character`, AI components, etc.).
///
/// Insert this on the **same entity** as the `AnimationPlayer`.
///
/// # Field
/// - `0`: The owner/root entity id.
#[derive(Component, Debug)]
pub struct AnimOwner(pub Entity);

/// This component lives on the **owner/root entity** (the one that represents the
/// model/character/NPC/prop). Systems that wire up animation (e.g., a
/// `bind_animations_to_entities` system) walk up the hierarchy from the
/// `AnimationPlayer` child, read this key, and use it to fetch the animation
/// graph and clip table from your asset registry (e.g. `LoadedAssets.animations`).
///
/// # Semantics
/// - Treat the value as a **stable identifier** for the model's animation set
///   (often the model or character name).
/// - It must match the key used in your animation asset map; otherwise binding
///   will be skipped.
/// - Set this on the **owner** entity, not on the `AnimationPlayer` child.
/// - If your binding system triggers only on `Added<AnimationPlayer>`, changing
///   this key at runtime will **not** automatically rebind existing players.
///
/// # Field
/// - `0`: The string identifier used to look up the animation set.
#[derive(Component, Debug)]
pub struct AnimKey(pub String);

/// Canonical, role-agnostic request describing *what should play now*.
///
/// Systems specific to each role (player/NPC/enemy/prop) should **write**
/// this, and a single apply-system reads it and drives the `AnimationPlayer`.
///
/// Keep it minimal and stable: avoid leaking gameplay details here.
#[derive(Component, Default, Clone, Debug)]
pub struct AnimIntent {
    /// Desired locomotion mode for this frame (optional if not applicable, e.g. props).
    pub locomotion: Option<Locomotion>,
}

/// Canonical locomotion categories understood by the animation layer.
///
/// This is intentionally small; blend details (speed, stride) should be
/// handled via clip speed/parameters or extended intent fields if needed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Locomotion { Idle, Walk, Run, Jump, SlowWalk }

/// Per-character clip layout (indices) used by the animation apply-system.
///
/// This decouples *what to play* from *where it lives* in the graph.
/// Populate it from your asset table (e.g. `LoadedAssets`) per character.
///
/// # Fields
/// - `idle_base`: Looping base idle clip index.
/// - `idle_variants`: Optional one-shot idle variants played occasionally.
/// - `walk`: Looping walk clip index.
/// - `run`: Looping run/sprint clip index.
/// - `jump`: Optional one-shot jump clip; if `None`, fall back to `idle_base`.
#[derive(Component, Clone, Debug)]
pub struct AnimSchema {
    pub idle_base: usize,
    pub idle_variants: &'static [usize],
    pub walk: usize,
    pub run: usize,
    pub slow_walk: usize,
    pub jump: Option<usize>,
}

/// Per-entity timer used to trigger idle variants at intervals.
///
/// Attach to the `AnimationPlayer` entity. The apply-system ticks this and
/// starts an idle variant when it finishes, then resets it.
#[derive(Component, Debug)]
pub struct IdleVariantTimer(pub Timer);

/// Runtime playback cache to avoid replay-spam and to let one-shots finish.
///
/// Managed by the apply-system; you should not modify this directly from
/// gameplay code.
///
/// # Fields
/// - `current_index`: The last clip index we switched to (for inexpensive “already playing?” checks).
/// - `one_shot_override`: If set, a one-shot clip (e.g., idle variant) has priority until it finishes.
#[derive(Component, Default, Debug)]
pub struct AnimPlayback {
    pub current_index: Option<usize>,
    pub one_shot_override: Option<usize>,
}