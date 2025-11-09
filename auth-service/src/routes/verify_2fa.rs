use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{app_state::AppState, domain::{data_store::{LoginAttemptId, TwoFACode, TwoFACodeStore}, email::Email, error::AuthAPIError}, services::{data_store::PostgresUserStore, mock_email_client::MockEmailClient, redis_banned_token_store::RedisBannedTokenStore, redis_two_fa_code_store::RedisTwoFACodeStore}, utils::auth::generate_auth_cookie};

pub async fn verify2fa(State(state): State<AppState<PostgresUserStore, RedisBannedTokenStore, RedisTwoFACodeStore, MockEmailClient>>,
    jar: CookieJar,
    Json(request): Json<VerifyRequest>) -> (CookieJar, impl IntoResponse) {
    // Because the function accepts a VerifyRequest Deserialized Json it will return
    // 422 if the Json sent with the request is malformed
    
    
    // return 400, bad request if email, login id or 2FA code are not valid 
    let email = match Email::parse(request.email) {
        Ok(e) => e,
        Err(_) => return (jar, StatusCode::BAD_REQUEST.into_response()),
    };
    let login_attempt_id = match LoginAttemptId::parse(request.loginattemptid) {
        Ok(id) => id,
        Err(_) =>  return (jar,StatusCode::BAD_REQUEST.into_response())
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(code) => code,
        Err(_) => return (jar, StatusCode::BAD_REQUEST.into_response()),
    };

    // read the login id and 2FAcode stored when client posts to /login successfully
    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    let (store_login_attempt_id, store_two_fa_code) = match two_fa_code_store.get_code(&email).await {
        Ok(tuple) => tuple,
        Err(_e) => return (jar, AuthAPIError::IncorrectCredentials.into_response())
    };

    // if they id and 2FA do not match what is in the store send back a 401
    if login_attempt_id != store_login_attempt_id || two_fa_code!= store_two_fa_code {
        return (jar, AuthAPIError::IncorrectCredentials.into_response())
    };

    // remove the 2FACode from the store
    let removal_result = two_fa_code_store.remove_code(&email).await;
    match removal_result {
        Ok(()) => {},
        Err(_) => return (jar, AuthAPIError::UnexpectedError.into_response())
    };
    // create a cookie
    let auth_cookie = generate_auth_cookie(email).map_err(|_|AuthAPIError::UnexpectedError);
    // if cookie has issue generating return an error with the empty jar
    let auth_cookie= match auth_cookie{
        Ok(cookie) => cookie,
        Err(e) => return (jar, e.into_response())
    };
    // If no error set the cookie in the jar
    let updated_jar = jar.add(auth_cookie);
    // return a 200 OK status 
    (updated_jar, StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    #[serde(rename = "LoginAttemptId")]
    pub loginattemptid: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
