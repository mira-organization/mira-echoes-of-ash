use std::time::{Duration, Instant};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_http_client::{HttpClient, HttpRequest, HttpResponse, HttpResponseError};
use game_system::save_info::PingData;

/// Plugin that handles measuring and tracking the client's ping (network latency)
/// to the backend server.
///
/// <p>This plugin sends periodic HTTP GET requests to the server and calculates
/// the time it takes for a response to return. The resulting latency is stored
/// in the [`PingData`] resource and can be used for display or diagnostics.</p>
pub struct NetworkPingService;

impl Plugin for NetworkPingService {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_resource::<PingData>();
        app.add_systems(Update, (handle_response, handle_error))
            .add_systems(
                Update,
                send_request.run_if(on_timer(Duration::from_millis(2500))),
            );
    }

}

/// Sends an HTTP GET request to the ping endpoint and records the time the request was sent.
///
/// <p>This system is triggered every 2.5 seconds by a timer condition. It writes
/// an HTTP request event to the Bevy event system and stores the timestamp of the request
/// in the [`PingData`] resource.</p>
#[coverage(off)]
fn send_request(mut ev_request: EventWriter<HttpRequest>, mut ping_data: ResMut<PingData>) {
    let request = HttpClient::new().get("http://85.215.116.15:8080/DEV/v0/status/ping").build();
    ev_request.write(request);
    let now = Instant::now();
    ping_data.last_request_time = Some(now);
}

/// Handles HTTP responses and calculates the ping duration from the last request.
///
/// <p>When an HTTP response is received, this system checks if a request timestamp exists,
/// calculates the elapsed time since the request, and stores it as the most recent ping
/// value in the [`PingData`] resource.</p>
#[coverage(off)]
fn handle_response(mut ev_resp: EventReader<HttpResponse>, mut ping_data: ResMut<PingData>) {
    for _ in ev_resp.read() {
        if let Some(start) = ping_data.last_request_time {
            let elapsed = start.elapsed();
            ping_data.last_ping = Some(elapsed);
        }
    }
}

/// Logs any errors encountered while attempting to send or receive ping requests.
///
/// <p>This system listens for [`HttpResponseError`] events and prints the error message
/// to the standard output. Useful for debugging connectivity issues.</p>
#[coverage(off)]
fn handle_error(mut ev_error: EventReader<HttpResponseError>) {
    for error in ev_error.read() {
        println!("Error retrieving Ping: {}", error.err);
    }
}