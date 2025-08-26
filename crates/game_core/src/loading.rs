#![coverage(off)]

use std::collections::{HashMap, HashSet};
use bevy::asset::UntypedAssetId;
use bevy::prelude::*;

/// Tracks the progress of asset loading within the Bevy application.
///
/// This resource keeps count of how many assets are expected (`total`),
/// how many have been successfully loaded (`loaded`), and any pending
/// handles that have not yet been typed/processed (`untyped_pending`).
#[derive(Resource, Default)]
pub struct AssetLoadProgress {
    /// The total number of assets that need to be loaded.
    pub total: usize,

    pub loaded: HashSet<AssetId<Scene>>,
}

/// Resource that holds references to all loaded assets in the Bevy world.
///
/// This resource aggregates three categories of assets:
/// 1. **Entities**: Loaded scene assets, keyed by a descriptive name.
/// 2. **Animations**: Animation graphs along with their node indices,
///    keyed by animation name.
/// 3. **Environments**: Untyped asset IDs for environment assets (e.g., skybox,
///    lighting setups) that have finished loading.
#[derive(Resource, Debug)]
pub struct LoadedAssets {
    /// A map from entity identifiers to their loaded `Scene` handles.
    pub entities: HashMap<String, Handle<Scene>>,

    /// A map from animation names to a tuple of:
    /// - `Handle<AnimationGraph>`: the graph asset for that animation, and
    /// - `Vec<AnimationNodeIndex>`: the indices of nodes within that graph.
    pub animations: HashMap<String, (Handle<AnimationGraph>, Vec<AnimationNodeIndex>)>,

    /// A list of untyped asset IDs representing loaded environment assets.
    pub environments: Vec<UntypedAssetId>,
}

/// Stores a handle to a GLTF asset that is being loaded for an area.
/// Used to track the loading state of area assets.
#[derive(Resource, Debug, Clone)]
pub struct WaitingForAreaAssets(pub Handle<Gltf>);

/// Stores a handle to a GLTF asset that contains effect scene data.
/// Used for loading additional scene effects like lights or particle effects.
#[derive(Resource, Debug, Clone)]
pub struct EffectSceneAssets(pub Handle<Gltf>);