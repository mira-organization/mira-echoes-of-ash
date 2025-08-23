use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse, TypedResponseError};
use game_core::network::authenticate::AuthResponse;
use game_core::network::save_resource::{SaveData, UserResponse};
use game_core::states::{AppState, FetchState, InGameStates};

pub struct SaveDataChannel;

impl Plugin for SaveDataChannel {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.register_request_type::<SaveData>();
        app.add_systems(Update, (handle_response, handle_error).run_if(in_state(AppState::NetworkFetch(FetchState::Fetching))))
            .add_systems(
                Update,
                send_request.run_if(in_state(AppState::NetworkFetch(FetchState::Fetching)))
                    .run_if(on_timer(std::time::Duration::from_secs(1))
                        .and(resource_changed::<UserResponse>)),
            );
        app.add_systems(Update, send_save_data_to_server.run_if(in_state(AppState::InGame(InGameStates::Game))
            .and(resource_changed::<SaveData>)));
    }
}

/// Sends an HTTP GET request to fetch the player's safe data from the server.
///
/// This function uses a `TypedRequest<SaveInfo>` to initiate a request via the
/// `HttpClient`. The response will later be handled by `handle_response`, and
/// any errors by `handle_error`.
///
/// # Parameters
/// - `ev_request`: An `EventWriter` that emits the typed HTTP request.
#[coverage(off)]
fn send_request(mut ev_request: EventWriter<TypedRequest<SaveData>>, user_entity: Res<UserResponse>, auth_data: Res<AuthResponse>) {
    ev_request.write(
        HttpClient::new()
            .get(format!("http://85.215.116.15:8080/REST/v0/api/saves/{}", user_entity.email).as_str())
            .headers(&[
                ("Authorization", format!("Bearer {}", auth_data.token).as_str()),
                ("Content-Type", "application/json")
            ])
            .try_with_type::<SaveData>().expect("REASON"),
    );
}


#[coverage(off)]
fn send_save_data_to_server(
    mut ev_request: EventWriter<TypedRequest<SaveData>>,
    user_entity: Res<UserResponse>,
    auth_data: Res<AuthResponse>,
    save_info: Res<SaveData>
) {
    debug!("Send new Save file to server!");
    ev_request.write(
        HttpClient::new()
            .post(format!("http://85.215.116.15:8080/REST/v0/api/saves/send/{}", user_entity.uid).as_str())
            .headers(&[
                ("Authorization", format!("Bearer {}", auth_data.token).as_str()),
                ("Content-Type", "application/json")
            ])
            .json(&save_info.clone())
            .try_with_type::<SaveData>().expect("REASON"),
    );
}

/// Handles successful HTTP responses for the save data request.
///
/// This function reads all available `TypedResponse<SaveInfo>` events,
/// extracts the inner `SaveInfo` value, and inserts it as a Bevy resource.
///
/// # Parameters
/// - `commands`: Bevy's `Commands` to insert resources.
/// - `events`: A mutable reference to the event queue of typed responses.
///
/// # Side Effects
/// Inserts a new `SaveInfo` resource into the Bevy world on success.
#[coverage(off)]
fn handle_response(mut commands: Commands, mut events: ResMut<Events<TypedResponse<SaveData>>>, mut next_game_state: ResMut<NextState<AppState>>) {
    for response in events.drain() {
        let response: SaveData = response.into_inner();
        commands.insert_resource(response.clone());
        debug!("Save Resource fetch successfully!");
        next_game_state.set(AppState::NetworkFetch(FetchState::FetchingComplete));
    }
}

/// Handles errors that occur during the HTTP save data request.
///
/// This function listens for `TypedResponseError<SaveInfo>` events and logs
/// the associated error messages for debugging or diagnostics purposes.
///
/// # Parameters
/// - `ev_error`: An event reader for HTTP response errors of type `SaveInfo`.
///
/// # Side Effects
/// Logs the error using the `error!` macro from Bevy's logging system.
#[coverage(off)]
fn handle_error(mut ev_error: EventReader<TypedResponseError<SaveData>>) {
    for error in ev_error.read() {
        error!("{:?}", error.response);
        error!("Error retrieving save data: {}", error.err);
    }
}