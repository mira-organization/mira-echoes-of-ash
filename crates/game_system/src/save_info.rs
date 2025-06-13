use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use crate::characters::Character;

/// Represents the core save data structure for a player.
///
/// This struct holds metadata about the player and their current party state.
/// Typically loaded from or saved to JSON, and used for both local and backend storage.
///
/// Fields include user identifiers and their current party members,
/// which are stored as a list of [`Character`] structs.
#[derive(Resource, Debug, Default, Serialize, Deserialize)]
pub struct SaveInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub birthday: String,
    pub party: Vec<Character>,
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
}

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