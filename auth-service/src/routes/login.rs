use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{data_store::UserStore, email::Email, error::AuthAPIError, password::Password}, services::{hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore}, utils::auth::generate_auth_cookie};



pub async fn login(State(state):State<AppState<HashmapUserStore, HashsetBannedTokenStore>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>)
{
    // Return 400 bad request for invalid email / password
    if Email::parse(request.email.clone()).is_err() || Password::parse(request.password.clone()).is_err() {
        return (jar, Err(AuthAPIError::InvalidCredentials))
                
    };
    // Get user store from the app's state
    let user_store = state.user_store.read().await;

    // let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    // let password = Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
    // if user does is not validated returen incorrectCredentials
    let user_validation = user_store.validate_user(&request.email,&request.password).await;

    if user_validation.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials))
    };
    // Call `user_store.get_user`.
    // Return AuthAPIError::IncorrectCredentials if the operation fails.
    let user = user_store.get_user(&request.email).await.map_err(|_| AuthAPIError::IncorrectCredentials);

    let user = match user {
        Ok(user) => user,
        Err(e) => return (jar, Err(e))
    };
    let email = Email::parse(request.email).map_err( |_| AuthAPIError::UnexpectedError);
    let email= match email{
        Ok(email) => email,
        Err(e) => return (jar, Err(e))
    };
    // Generate cookie
    let auth_cookie = generate_auth_cookie(email).map_err(|_|AuthAPIError::UnexpectedError);
    let auth_cookie= match auth_cookie{
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(e))
    };

    let updated_jar = jar.add(auth_cookie);
    (updated_jar, Ok(StatusCode::OK.into_response()))
    
    
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
