#![coverage(off)]

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A resource representing the response returned by the authentication endpoint.
///
/// This struct is both serializable and deserializable, allowing it to be used as
/// a shared data structure for JSON-based HTTP communication. It includes
/// metadata such as creation timestamps and account status.
#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
}

/// A resource representing authentication input data provided by the user.
///
/// This struct is serialized to JSON and sent in the login request body.
/// The `username` field is renamed to `"email"` to match backend expectations.
#[derive(Resource, Debug, Default, Serialize, Clone)]
pub struct AuthData {
    /// The email address used as the username for login.
    #[serde(rename = "email")]
    pub username: String,

    /// The plain-text password for authentication.
    pub password: String,
}