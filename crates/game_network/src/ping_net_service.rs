use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use bevy::prelude::*;
use game_system::save_info::PingData;

#[derive(Resource)]
struct PingTimer(Timer);

/// A plugin responsible for managing periodic UDP-based ping communication with a server.
///
/// This plugin sets up a UDP socket, sends timestamped ping messages at fixed intervals,
/// and listens for pong responses to calculate round-trip time (RTT).
pub struct NetworkPingService;

impl Plugin for NetworkPingService {

    /// Initializes the ping socket, resources, and systems.
    ///
    /// - Initializes `PingData` and a repeating `PingTimer`.
    /// - Sets up the UDP socket during startup.
    /// - Adds systems to send pings and receive pong responses during the update loop.
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_resource::<PingData>();
        app.insert_resource(PingTimer(Timer::from_seconds(2.5, TimerMode::Repeating)));
        app.add_systems(Startup, setup_socket);
        app.add_systems(Update, (send_ping, receive_pong));
    }

}

/// Binds and connects a non-blocking UDP socket for ping communication.
///
/// The socket is bound to a random local port (`0.0.0.0:0`) and connected to the server.
/// It is then stored in the `PingData` resource.
#[coverage(off)]
fn setup_socket(mut ping_data: ResMut<PingData>) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("UDP Socket bind failed");
    
    socket
        .connect("85.215.116.15:14191")
        .expect("UDP Socket connect failed");
    
    socket.set_nonblocking(true).expect("Set nonblocking failed");

    ping_data.socket = Some(socket);
    info!("Created UDP socket for ping requests!");
}

/// Sends a ping message over UDP at regular intervals defined by `PingTimer`.
///
/// The ping contains the current system time (in milliseconds) as an 8-byte payload,
/// prefixed by a `1` byte identifier. This timestamp will be used to calculate RTT
/// when the corresponding pong is received.
#[coverage(off)]
fn send_ping(time: Res<Time>, mut timer: ResMut<PingTimer>, mut ping_data: ResMut<PingData>) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    if let Some(socket) = &ping_data.socket {
        // Timestamp in ms, als u64 (8 Bytes)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        let mut buf = [0u8; 9];
        buf[0] = 1; 
        buf[1..9].copy_from_slice(&now.to_le_bytes());

        if let Err(e) = socket.send(&buf) {
            eprintln!("UDP send error: {}", e);
        } else {
            ping_data.last_ping = Some(now as u128);
        }
    }
}

/// Listens for a pong response and calculates the round-trip time (RTT).
///
/// A pong is expected to be 9 bytes long, beginning with byte identifier `2`.
/// The payload contains the original ping timestamp, which is used to compute RTT
/// when compared to the current system time.
#[coverage(off)]
fn receive_pong(mut ping_data: ResMut<PingData>) {
    if let Some(socket) = &ping_data.socket {
        let mut buf = [0u8; 9];

        match socket.recv(&mut buf) {
            Ok(received) if received == 9 && buf[0] == 2 => {
                let sent_timestamp = u64::from_le_bytes(buf[1..9].try_into().unwrap());

                if let Some(_) = ping_data.last_ping {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis() as u64;

                    let rtt_millis = now.saturating_sub(sent_timestamp);
                    ping_data.last_rtt = Some(Duration::from_millis(rtt_millis));
                }
            }
            Ok(_) => {}
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            }
            Err(e) => {
                eprintln!("UDP recv error: {}", e);
            }
        }
    }
}