use std::collections::HashSet;
use bevy::prelude::*;
use game_core::loading::{AssetLoadProgress, LoadedAssets};
use game_core::states::{is_state_loading, AppState, AssetLoadState};

pub struct LoadStateService;

impl Plugin for LoadStateService {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::AssetsLoad(AssetLoadState::EnvPreLoad)), calc_total_assets);
        app.add_systems(Update, check_asset_status.run_if(is_state_loading));
    }
}

/// Computes and stores the total number of unique assets to load.
///
/// This system collects all untyped handle IDs for entity scenes and
/// environment assets, deduplicates them, and initializes the
/// `AssetLoadProgress` resource with the total count and an empty set
/// of loaded assets.
///
/// # Parameters
///
/// - `commands`: Command buffer used to insert the `AssetLoadProgress` resource.
/// - `assets`: Read‐only resource containing handles for entity scenes (`entities`)
///   and environment assets (`environments`).
#[coverage(off)]
fn calc_total_assets(
    mut commands: Commands,
    assets: Res<LoadedAssets>
) {
    let mut handle_ids = HashSet::new();
    for handle in assets.entities.values() {
        let untyped = handle.clone().untyped();
        handle_ids.insert(untyped.id());
    }

    for handle_id in assets.environments.iter() {
        handle_ids.insert(*handle_id);
    }

    commands.insert_resource(AssetLoadProgress {
        total: handle_ids.len(),
        loaded: HashSet::new()
    });
}

/// Listens for scene asset events to update load progress and transition state.
///
/// This system reads `AssetEvent<Scene>` events and:
/// - Inserts newly added asset IDs into the `loaded` set of `AssetLoadProgress`.
/// - Logs any `Unused` events for debugging.
/// Once the number of loaded assets exceeds the recorded total, it logs
/// completion and advances the `AppState` to `PostLoad`.
///
/// # Parameters
///
/// - `events`: Event reader for `AssetEvent<Scene>`, capturing asset lifecycle events.
/// - `progress`: Mutable resource tracking the total expected assets and those loaded so far.
/// - `next_state`: Mutable resource used to schedule a transition to `AppState::PostLoad`
///   when loading is complete.
#[coverage(off)]
fn check_asset_status(
    mut events: EventReader<AssetEvent<Scene>>,
    mut progress: ResMut<AssetLoadProgress>,
    mut next_state: ResMut<NextState<AppState>>
) {
    for event in events.read() {
        if let AssetEvent::Added { id } = event {
            progress.loaded.insert(*id);
        }

        if let AssetEvent::Unused { id } = event {
            debug!("WARN!!! Unused asset: {}", id);
        }
    }

    if progress.loaded.len() > progress.total && progress.total > 0 {
        info!("All assets loaded");
        next_state.set(AppState::PostLoad);
    }
}