use bevy::prelude::*;
use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse, TypedResponseError};
use game_core::network::authenticate::AuthResponse;
use game_core::network::save_resource::{UserResponse};
use game_core::states::{AppState, FetchState};

pub struct UserDataChannel;

impl Plugin for UserDataChannel {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.register_request_type::<UserResponse>();
        app.add_systems(Update, (handle_response, handle_error).run_if(in_state(AppState::NetworkFetch(FetchState::Fetching))))
            .add_systems(Update, send_user_request
                .run_if(in_state(AppState::NetworkFetch(FetchState::Fetching))
                .and(resource_changed::<AuthResponse>)));
    }
}

/// Sends an HTTP request to fetch the current user data.
///
/// This system function constructs a GET request targeting the
/// `/REST/v0/api/users/get` endpoint, attaches the bearer token and
/// JSON-serialized authentication data in the headers and body,
/// and emits it as a `TypedRequest<UserResponse>` event.
///
/// # Parameters
///
/// - `ev_request`: Event writer for `TypedRequest<UserResponse>`. The
///   prepared request is written into this event channel for processing
///   by the HTTP client system.
/// - `auth_data`: Read-only resource containing the authentication
///   response (`AuthResponse`), from which the bearer token is
///   extracted and serialized into the request body.
///
/// # Panics
///
/// This function will panic if the request builder’s
/// `try_with_type::<UserResponse>()` fails to generate a properly
/// typed request (e.g., serialization error), with the message `"REASON"`.
#[coverage(off)]
fn send_user_request(mut ev_request: EventWriter<TypedRequest<UserResponse>>, auth_data: Res<AuthResponse>) {
    ev_request.write(
        HttpClient::new()
            .get("http://85.215.116.15:8080/REST/v0/api/users/get")
            .headers(&[
                ("Authorization", format!("Bearer {}", auth_data.token).as_str()),
                ("Content-Type", "application/json")
            ])
            .json(&auth_data.clone())
            .try_with_type::<UserResponse>().expect("REASON"),
    );
}

/// Handles incoming `UserResponse` events and makes the data available
/// as a Bevy resource.
///
/// This system drains all pending `TypedResponse<UserResponse>` events,
/// extracts the inner `UserResponse` payload, inserts a clone of it
/// into the Bevy world as a resource, and logs the received username.
///
/// # Parameters
///
/// - `commands`: Commands buffer to insert the extracted `UserResponse`
///   into the world as a resource.
/// - `events`: Mutable resource wrapping the `Events<TypedResponse<UserResponse>>`
///   channel. All queued responses are drained in this function.
///
/// # Behavior
///
/// For each response event:
/// 1. Convert the `TypedResponse<UserResponse>` into its inner `UserResponse`.
/// 2. Insert a clone of that `UserResponse` as a global resource.
/// 3. Emit a debug log with the received username.
#[coverage(off)]
fn handle_response(mut commands: Commands, mut events: ResMut<Events<TypedResponse<UserResponse>>>) {
    for response in events.drain() {
        let response: UserResponse = response.into_inner();
        commands.insert_resource(response.clone());
        debug!("Received user : ( {:?} )", response.username);
    }
}

/// Handles errors that occur during the authentication request.
///
/// This system listens for `TypedResponseError<AuthResponse>` events and
/// logs both the response (if available) and the underlying error message.
///
/// Only runs when in the `AccountScreen` state.
#[coverage(off)]
fn handle_error(mut ev_error: EventReader<TypedResponseError<UserResponse>>) {
    for error in ev_error.read() {
        error!("{:?}", error.response);
        error!("Error retrieving user entity: {}", error.err);
    }
}