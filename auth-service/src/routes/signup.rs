use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{error::AuthAPIError, user::User}};
// Order of parameters is important in the handler
pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>, ) -> Result<impl IntoResponse, AuthAPIError> {

    let email = request.email;
    let password = request.password;
    if email.is_empty() || !email.contains("@") || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    };
    let user = User {
        email: email.clone(),
        password,
        requires_2fa : request.requires_2fa,
    };

    // lock the store first before writing data into it
    let mut user_store = state.user_store.write().await;
    // Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    
    // if user already in store return error
    if user_store.users.contains_key(&email) {
        return Err(AuthAPIError::UserAlreadyExists);
    }
    match user_store.add_user(user) {
        Ok(()) => {
            
            let response = Json( SignupResponse {
                message: "User created successfully!".to_string(),
            });    
            Ok((StatusCode::CREATED, response))
        },
        Err(_) => Err(AuthAPIError::UnexpectedError),
    }

}


#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize,Clone, Debug, PartialEq, PartialOrd)]
pub struct SignupResponse {
    pub message: String,
}
