use bevy::prelude::*;
use bevy_extended_ui::registry::UiRegistry;
use game_core::entities::character::ChangeCharacter;
use game_core::states::{AppState, InGameStates};

pub struct PostLoadService;

impl Plugin for PostLoadService {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::PostLoad), spawn_player_to_world);
    }
}

/// Spawns the selected player into the game world and transitions to the main gameplay state.
///
/// This system sets the `ChangeCharacter` flag to `true` to trigger character loading,
/// clears the current UI by selecting an empty UI key, and enqueues a transition
/// to `AppState::InGame(InGameStates::Game)`.
///
/// # Parameters
///
/// - `change_character`: Mutable resource flag indicating that a character change/spawn should occur.
/// - `next_state`: Mutable resource used to schedule the state transition to the in-game state.
/// - `ui_registry`: Mutable UI registry resource for activating or deactivating UI screens.
#[coverage(off)]
fn spawn_player_to_world(
    mut change_character: ResMut<ChangeCharacter>,
    mut next_state: ResMut<NextState<AppState>>,
    mut ui_registry: ResMut<UiRegistry>,
) {
    change_character.0 = true;
    ui_registry.use_ui("");
    next_state.set(AppState::InGame(InGameStates::Game));
}