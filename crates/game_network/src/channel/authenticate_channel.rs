use bevy::prelude::*;
use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse, TypedResponseError};
use game_core::network::authenticate::{AuthData, AuthResponse};
use game_core::states::{AppState, BeforeUiState};

pub struct AuthenticateChannel;

impl Plugin for AuthenticateChannel {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.register_request_type::<AuthResponse>();
        app.add_systems(Update, (handle_response, handle_error).run_if(in_state(AppState::Screen(BeforeUiState::Account))))
            .add_systems(
                Update,
                send_login_request.run_if(in_state(AppState::Screen(BeforeUiState::Account))).run_if(resource_changed::<AuthData>)
            );
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
        debug!("Authentication successfully!");
        commands.insert_resource(response.clone());
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
    }
}