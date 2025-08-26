mod authenticate_channel;
mod save_data_channel;
mod user_data_channel;
mod ping_channel;

use bevy::prelude::*;
use crate::channel::authenticate_channel::AuthenticateChannel;
use crate::channel::ping_channel::PingChannel;
use crate::channel::save_data_channel::SaveDataChannel;
use crate::channel::user_data_channel::UserDataChannel;

pub struct NetworkChannelModule;

impl Plugin for NetworkChannelModule {
    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AuthenticateChannel, 
            UserDataChannel, 
            SaveDataChannel, 
            PingChannel
        ));
    }
}