use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse, TypedResponseError};
use game_system::app_state::GameState;
use game_system::save_info::SaveInfo;

pub struct SaveDataService;

impl Plugin for SaveDataService {
    
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_response, handle_error).run_if(in_state(GameState::SplashScreen)))
            .add_systems(
                Update,
                send_request.run_if(in_state(GameState::SplashScreen)).run_if(on_timer(std::time::Duration::from_secs(1))),
            );
        app.register_request_type::<SaveInfo>();
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
fn send_request(mut ev_request: EventWriter<TypedRequest<SaveInfo>>) {
    ev_request.write(
        HttpClient::new()
            .get("http://85.215.116.15:8080/REST/v0/api/save/dummy")
            .try_with_type::<SaveInfo>().expect("REASON"),
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
fn handle_response(mut commands: Commands, mut events: ResMut<Events<TypedResponse<SaveInfo>>>) {
    for response in events.drain() {
        let response: SaveInfo = response.into_inner();
        commands.insert_resource(response.clone());
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
fn handle_error(mut ev_error: EventReader<TypedResponseError<SaveInfo>>) {
    for error in ev_error.read() {
        info!("{:?}", error.response);
        error!("Error retrieving save data: {}", error.err);
    }
}