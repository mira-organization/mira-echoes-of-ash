use bevy::prelude::*;
use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse, TypedResponseError};
use game_system::app_state::GameState;
use game_system::save_info::{AuthResponse, UserEntity};

pub struct UserService;

impl Plugin for UserService {

    #[coverage(off)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_response, handle_error).run_if(in_state(GameState::FetchUserData)))
            .add_systems(Update, send_user_request.run_if(in_state(GameState::FetchUserData).and(resource_changed::<AuthResponse>)));
        app.register_request_type::<UserEntity>();
    }
}

#[coverage(off)]
fn send_user_request(mut ev_request: EventWriter<TypedRequest<UserEntity>>, auth_data: Res<AuthResponse>) {
    ev_request.write(
        HttpClient::new()
            .get("http://85.215.116.15:8080/REST/v0/api/users/get")
            .headers(&[
                ("Authorization", format!("Bearer {}", auth_data.token).as_str()),
                ("Content-Type", "application/json")
            ])
            .json(&auth_data.clone())
            .try_with_type::<UserEntity>().expect("REASON"),
    );
}

#[coverage(off)]
fn handle_response(mut commands: Commands, mut events: ResMut<Events<TypedResponse<UserEntity>>>, mut next_game_state: ResMut<NextState<GameState>>) {
    for response in events.drain() {
        let response: UserEntity = response.into_inner();
        commands.insert_resource(response.clone());
        debug!("{:?}", response);
        next_game_state.set(GameState::FetchSaveData);
    }
}

/// Handles errors that occur during the authentication request.
///
/// This system listens for `TypedResponseError<AuthResponse>` events and
/// logs both the response (if available) and the underlying error message.
///
/// Only runs when in the `AccountScreen` state.
#[coverage(off)]
fn handle_error(mut ev_error: EventReader<TypedResponseError<UserEntity>>) {
    for error in ev_error.read() {
        error!("{:?}", error.response);
        error!("Error retrieving user entity: {}", error.err);
    }
}