use bevy::prelude::*;
use crate::network::authenticate::{AuthData, AuthResponse};
use crate::network::ping_resource::PingResponse;
use crate::network::save_resource::UserResponse;

pub mod authenticate;
pub mod save_resource;
pub mod ping_resource;

pub struct NetworkModule;

impl Plugin for NetworkModule {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.init_resource::<PingResponse>();
        app.init_resource::<AuthData>();
        app.init_resource::<AuthResponse>();
        app.init_resource::<UserResponse>();
    }
}