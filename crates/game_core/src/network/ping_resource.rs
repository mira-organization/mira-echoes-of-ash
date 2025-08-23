#![coverage(off)]

use std::net::UdpSocket;
use std::time::Duration;
use bevy::prelude::*;

/// Represents the current network ping state of the client.
///
/// This resource stores the latest ping duration between the game client and the server,
/// as well as the timestamp of when the last request was sent. This data can be used
/// to display ping statistics in the UI or for debugging network latency issues.
#[derive(Resource, Debug, Default)]
pub struct PingResponse {
    pub last_ping: Option<u128>,
    pub last_rtt: Option<Duration>,
    pub socket: Option<UdpSocket>
}