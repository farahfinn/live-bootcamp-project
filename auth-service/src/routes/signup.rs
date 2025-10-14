use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::user::User};
// Order of parameters is important in the handler
pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>, ) -> impl IntoResponse {
    let user = User {
        email : request.email,
        password : request.password,
        requires_2fa : request.requires_2fa,
    };

    // lock the store first before writing data into it
    let mut user_store = state.user_store.write().await;
    // TODO: Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    user_store.add_user(user).unwrap();

    let response = Json( SignupResponse {
        message: "User created successfully!".to_string(),
    });    
    (StatusCode::CREATED, response)
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
