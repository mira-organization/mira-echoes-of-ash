use bevy::prelude::*;
use bevy_extended_ui::registry::UiRegistry;
use bevy_extended_ui::styling::convert::CssID;
use bevy_extended_ui::widgets::InputField;
use game_core::network::authenticate::{AuthData, AuthResponse};
use game_core::states::{AppState, BeforeUiState, FetchState};

#[derive(Component)]
struct ObserverRegistered;

/// A Bevy plugin responsible for rendering and managing the account login screen.
///
/// This plugin displays the account login UI, sets up observers for login-related buttons,
/// and handles transitions after authentication. It registers systems that run only while
/// the game is in the `GameState::AccountScreen` state.
pub struct AccountScreen;

impl Plugin for AccountScreen {

    /// Builds and registers the systems for the `AccountScreen` plugin.
    ///
    /// - `display_account_screen`: Loads the HTML UI for the login screen on state entry.
    /// - `control_debug_login`: Observes a debug login button that bypasses authentication.
    /// - `control_login`: Observes the actual login button and reads user input fields.
    /// - `check_response`: Reacts to changes in authentication state and switches game state.
    /// - `hide_account_screen`: Cleans up the login screen UI on state exit.
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Screen(BeforeUiState::Account)), display_account_screen);
        app.add_systems(Update, control_debug_login.run_if(in_state(AppState::Screen(BeforeUiState::Account))));
        app.add_systems(Update, control_login.run_if(in_state(AppState::Screen(BeforeUiState::Account))));
        app.add_systems(Update, check_response
            .run_if(in_state(AppState::Screen(BeforeUiState::Account)))
            .run_if(resource_changed::<AuthResponse>)
        );
    }
}

/// Spawns the account screen UI by loading the `account.html` file.
///
/// Called once when entering the `AccountScreen` state.
#[coverage(off)]
fn display_account_screen(mut ui_registry: ResMut<UiRegistry>) {
    ui_registry.use_ui("account_screen");
}

/// Adds an observer to the debug login button with ID `"debug-login"`.
///
/// When clicked, the game skips authentication and transitions directly to `GameState::Preload`.
/// Only applies if the button has not already been registered as an observer.
#[coverage(off)]
fn control_debug_login(mut commands: Commands, query: Query<(Entity, &CssID), Without<ObserverRegistered>>) {
    for (entity, id) in query.iter() {
        if id.0.eq("debug-login") {
            commands.entity(entity)
                .insert(ObserverRegistered)
                .observe(|_: Trigger<Pointer<Click>>, mut auth_data: ResMut<AuthData>| {
                    auth_data.username = "dev@tilt-us.com".to_string();
                    auth_data.password = "dev123456".to_string();
                });
        }
    }
}

/// Adds an observer to the login submit button with ID `"submit"`.
///
/// When clicked, this system reads the input fields with IDs `"username"` and `"password"`
/// and updates the global `AuthData` resource. Triggers an HTTP request if `AuthData` changes.
/// Only applies if the button has not already been registered as an observer.
#[coverage(off)]
fn control_login(mut commands: Commands, query: Query<(Entity, &CssID), Without<ObserverRegistered>>) {
    for (entity, id) in query.iter() {
        if id.0.eq("submit") {
            commands.entity(entity)
                .insert(ObserverRegistered)
                .observe(|_: Trigger<Pointer<Click>>, mut auth_data: ResMut<AuthData>, inner_query: Query<(&CssID, &InputField)> | {
                    for (inner_id, in_field) in inner_query.iter() {
                        if inner_id.0.eq("username") {
                            auth_data.username = in_field.text.clone();
                        }
                        if inner_id.0.eq("password") {
                            auth_data.password = in_field.text.clone();
                        }
                    }
                    debug!("authenticate user: ( {} )", auth_data.username);
                });
        }
    }
}

/// Checks for a successful login response and transitions to the preload state.
///
/// Triggered when the `AuthResponse` resource is updated, indicating that a response was received.
#[coverage(off)]
fn check_response(mut next_game_state: ResMut<NextState<AppState>>) {
    next_game_state.set(AppState::NetworkFetch(FetchState::Fetching));
}