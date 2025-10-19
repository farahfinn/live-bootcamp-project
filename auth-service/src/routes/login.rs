use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{data_store::UserStore, email::Email, error::AuthAPIError, password::Password}, services::hashmap_user_store::HashmapUserStore};



pub async fn login(State(state):State<AppState<HashmapUserStore>>,Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError>
{
    // Return 400 bad request for invalid email / password
    if Email::parse(request.email.clone()).is_err() || Password::parse(request.password.clone()).is_err() {
        return Err(AuthAPIError::InvalidCredentials)
                
    };
    // Get user store from the app's state
    let user_store = state.user_store.read().await;

    // let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    // let password = Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
    // if user does is not validated returen incorrectCredentials
    user_store.validate_user(&request.email,&request.password).await.map_err(|_| AuthAPIError::IncorrectCredentials)?;
    // Call `user_store.get_user`.
    // Return AuthAPIError::IncorrectCredentials if the operation fails.
    let user = user_store.get_user(&request.email).await.map_err(|_| AuthAPIError::IncorrectCredentials)?;

        
    let response = Json(LoginResponse{message: "Login successful".to_string()});
    Ok((StatusCode::OK, response))
    
    
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct LoginResponse {
    pub message: String
}
