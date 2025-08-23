use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use crate::entities::character::Character;
use crate::entities::item::Item;

/// Represents the core save data structure for a player.
///
/// This struct holds metadata about the player and their current party state.
/// Typically loaded from or saved to JSON and used for both local and backend storage.
///
/// Fields include user identifiers and their current party members,
/// which are stored as a list of [`Character`] structs.
#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub id: String,
    pub username: String,
    pub email: String,
    pub party: Vec<Character>,
    pub current_environment: String,
    pub current_area: usize,
    pub location: PlayerLocation,
    pub items: Vec<Item>
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PlayerLocation {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl SaveData {
    /// Parses a JSON string into a SaveInfo struct.
    /// Returns a Result to provide error details if parsing fails.
    pub fn fetch_from_json(json: &String) -> Result<SaveData, Error> {
        serde_json::from_str(json)
    }
}

#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    /// Unique identifier of the user.
    pub uid: usize,

    /// The username or display name associated with the user.
    pub username: String,

    /// The user's registered email address.
    pub email: String,

    /// The hashed password returned from the server.
    pub password: String,

    /// The user's birthday, which may be `null`.
    pub birthday: Option<String>,

    /// The ISO timestamp when the account was created.
    #[serde(rename = "createdDate")]
    pub created_date: String,

    /// The ISO timestamp when the account was last updated.
    #[serde(rename = "updateDate")]
    pub update_date: String,

    /// The status of the account (e.g., `"created"`, `"active"`, `"banned"`).
    pub status: String,
}