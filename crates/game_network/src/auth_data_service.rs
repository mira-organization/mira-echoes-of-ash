use bevy::prelude::*;
use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse, TypedResponseError};
use game_system::app_state::GameState;
use game_system::save_info::{AuthData, AuthResponse};

/// A Bevy plugin that handles user authentication via HTTP POST requests.
///
/// This plugin registers systems for sending login requests and handling responses
/// when the game is in the `GameState::AccountScreen` state. It also registers the
/// expected response type `AuthResponse` for deserialization.
pub struct AuthDataService;

impl Plugin for AuthDataService {

    /// Builds and registers the systems for the `AuthDataService` plugin.
    ///
    /// - `send_request`: Sends the authentication HTTP POST request.
    /// - `handle_response`: Handles the successful response and stores the `AuthResponse` as a resource.
    /// - `handle_error`: Logs any errors encountered during the request.
    ///
    /// These systems only run when the game is in the `GameState::AccountScreen`
    /// and when the `AuthData` resource has changed (for `send_request`).
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_response, handle_error).run_if(in_state(GameState::AccountScreen)))
            .add_systems(
                Update,
                send_login_request.run_if(in_state(GameState::AccountScreen)).run_if(resource_changed::<AuthData>)
            );
        app.register_request_type::<AuthResponse>();
    }
}

/// Sends an HTTP POST request to the authentication endpoint using the current `AuthData`.
///
/// This system serializes the `AuthData` into JSON and posts it to the backend login API.
/// It emits a typed request event that triggers a typed response (`AuthResponse`)
/// or an error if the request fails.
///
/// Runs only when the `AuthData` resource has changed and the state is `AccountScreen`.
#[coverage(off)]
fn send_login_request(mut ev_request: EventWriter<TypedRequest<AuthResponse>>, auth_data: Res<AuthData>) {
    ev_request.write(
        HttpClient::new()
            .post("http://85.215.116.15:8080/REST/v0/api/auth/login")
            .headers(&[
                ("Content-Type", "application/json")
            ])
            .json(&auth_data.clone())
            .try_with_type::<AuthResponse>().expect("REASON"),
    );
}

/// Handles successful HTTP authentication responses.
///
/// This system listens for `TypedResponse<AuthResponse>` events and inserts
/// the deserialized response as a Bevy resource. This makes the authenticated
/// user data globally available within the app.
///
/// Only runs when in the `AccountScreen` state.
#[coverage(off)]
fn handle_response(mut commands: Commands, mut events: ResMut<Events<TypedResponse<AuthResponse>>>) {
    for response in events.drain() {
        let response: AuthResponse = response.into_inner();
        commands.insert_resource(response.clone());
        debug!("{:?}", response);
    }
}

/// Handles errors that occur during the authentication request.
///
/// This system listens for `TypedResponseError<AuthResponse>` events and
/// logs both the response (if available) and the underlying error message.
///
/// Only runs when in the `AccountScreen` state.
#[coverage(off)]
fn handle_error(mut ev_error: EventReader<TypedResponseError<AuthResponse>>) {
    for error in ev_error.read() {
        error!("{:?}", error.response);
        error!("Error retrieving auth token: {}", error.err);
    }
}