use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{data_store::UserStore, email::Email, error::AuthAPIError, password::Password, user::User}, services::{hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore, mock_email_client::MockEmailClient}};
// Order of parameters is important in the handler
pub async fn signup(State(state): State<AppState<HashmapUserStore, HashsetBannedTokenStore, HashmapTwoFACodeStore, MockEmailClient>>,Json(request): Json<SignupRequest> ) -> Result<impl IntoResponse, AuthAPIError> {

    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
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
    match user_store.add_user(user).await {
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
