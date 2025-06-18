use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::Duration;
use bevy::asset::UntypedAssetId;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use crate::characters::Character;
use crate::models::logic::JSONCharacter;

/// Represents the current network ping state of the client.
///
/// This resource stores the latest ping duration between the game client and the server,
/// as well as the timestamp of when the last request was sent. This data can be used
/// to display ping statistics in the UI or for debugging network latency issues.
#[derive(Resource, Debug, Default)]
pub struct PingData {
    pub last_ping: Option<u128>,
    pub last_rtt: Option<Duration>,
    pub socket: Option<UdpSocket>
    
}

/// Represents the core save data structure for a player.
///
/// This struct holds metadata about the player and their current party state.
/// Typically loaded from or saved to JSON and used for both local and backend storage.
///
/// Fields include user identifiers and their current party members,
/// which are stored as a list of [`Character`] structs.
#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct SaveInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub birthday: String,
    pub party: Vec<Character>,
    pub current_environment: String,
    pub current_area: usize,
}

impl SaveInfo {
    /// Parses a JSON string into a SaveInfo struct.
    /// Returns a Result to provide error details if parsing fails.
    pub fn fetch_from_json(json: &String) -> Result<SaveInfo, Error> {
        serde_json::from_str(json)
    }
}

#[derive(Resource, Debug)]
pub struct LoadedAssets {
    pub characters: HashMap<String, Handle<Scene>>,
    pub animations: HashMap<String, (Handle<AnimationGraph>, Vec<AnimationNodeIndex>)>,
    pub environments: Vec<UntypedAssetId>
}

/// A resource indicating whether the player wants to switch characters.
#[derive(Resource, Default, Clone, Debug)]
pub struct ChangeCharacter(pub bool);

/// A resource storing the currently active world character.
#[derive(Resource, Default, Clone, Debug)]
pub struct CurrentWorldCharacter(pub Option<(Entity, Character)>);

#[derive(Resource, Default, Clone, Debug)]
pub struct AllCharacters(pub Vec<JSONCharacter>);

// ================================================================
//                               Tests
// ================================================================

#[cfg(test)]
mod unit_tests {
    use crate::save_info::SaveInfo;

    #[test]
    fn test_fetch_from_valid_json_file() {
        let json = include_str!("../../../dummy/rest-save.json");
        let result = SaveInfo::fetch_from_json(&json.to_string());
        assert!(result.is_ok(), "Expected valid JSON to parse successfully");

        let save_info = result.unwrap();
        println!("Loaded save for user: {}", save_info.username);
        assert!(!save_info.party.is_empty(), "Party should not be empty");
    }

    #[test]
    fn test_fetch_from_invalid_json() {
        let invalid_json = r#"{
        "id": "abc",
        "username": "mira"
        "email": "invalid@example.com"
    }"#;

        let result = SaveInfo::fetch_from_json(&invalid_json.to_string());
        assert!(result.is_err(), "Expected parsing to fail with invalid JSON");

        if let Err(err) = result {
            println!("Error message: {}", err);
        }
    }
    
}