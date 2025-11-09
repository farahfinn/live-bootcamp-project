use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{data_store::{LoginAttemptId, TwoFACode, TwoFACodeStore, UserStore}, email::Email, email_client::EmailClient, error::AuthAPIError, password::Password}, services::{data_store::PostgresUserStore, mock_email_client::MockEmailClient, redis_banned_token_store::RedisBannedTokenStore, redis_two_fa_code_store::RedisTwoFACodeStore}, utils::auth::generate_auth_cookie};



pub async fn login(State(state):State<AppState<PostgresUserStore, RedisBannedTokenStore, RedisTwoFACodeStore, MockEmailClient>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>)
{
    // Return 400 bad request for invalid email / password
    // Get user store from the app's state
    let user_store = state.user_store.read().await;

    // parse the email & password, they should be able to now since they passed the if block above
    let email = Email::parse(request.email).map_err( |_| AuthAPIError::InvalidCredentials);
    let password = Password::parse(request.password).map_err( |_| AuthAPIError::InvalidCredentials);
    // check if email & password get parsed correctly return error
    let email= match email{
        Ok(email) => email,
        Err(e) => return (jar, Err(e))
    };

    let password= match password{
        Ok(password) => password,
        Err(e) => return (jar, Err(e))
    };
    
    // if user is not validated return incorrectCredentials
    let user_validation = user_store.validate_user(email.as_ref(),password.as_ref()).await;

    if user_validation.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials))
    };
    // Call `user_store.get_user`.
    // Return AuthAPIError::IncorrectCredentials if the operation fails.
    let user = user_store.get_user(email.as_ref()).await.map_err(|_| AuthAPIError::IncorrectCredentials);

    let user = match user {
        Ok(user) => user,
        Err(e) => return (jar, Err(e))
    };

    // handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(jar, user.email, &state).await,
        false => handle_no_2fa(user.email, jar).await
    }
    
    
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

async fn handle_2fa(jar: CookieJar,
    email: Email,
    state: &AppState<PostgresUserStore, RedisBannedTokenStore, RedisTwoFACodeStore, MockEmailClient>) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    //Create the cookie using email
    let auth_cookie = generate_auth_cookie(email.clone()).map_err(|_|AuthAPIError::UnexpectedError);

    // Generate random login attempt ID & 2FA Code
    let login_attempt_id = LoginAttemptId::default();

    let code = TwoFACode::default();

    // get the codes store
    let mut two_fa_codes_store = state.two_fa_code_store.write().await;

    // check if the code is added successfully to store
    match two_fa_codes_store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await {
        Ok(()) => { },
        Err(_e) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    // Send 2FA code via email client. return AuthApiError if operation fails
    let email_send_result = MockEmailClient.send_email(&email, login_attempt_id.as_ref(), code.as_ref()).await;
    match email_send_result {
        Ok(())=> {},
        Err(_e) => return (jar, Err(AuthAPIError::UnexpectedError))
    }    
    // if cookie has issue generating return an error with the empty jar
    let auth_cookie= match auth_cookie{
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(e))
    };
    // If no error set the cookie in the jar
    let updated_jar = jar.add(auth_cookie);
    let two_fa_auth_response = TwoFactorAuthResponse {message: "2FA required".into(), login_attempt_id: login_attempt_id.as_ref().into()};
    (updated_jar, Ok((StatusCode::PARTIAL_CONTENT,Json(LoginResponse::TwoFactorAuth(two_fa_auth_response)))))
}

async fn handle_no_2fa(email: Email, jar: CookieJar)->(CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>)  {
    
    //Create the cookie using email
    let auth_cookie = generate_auth_cookie(email).map_err(|_|AuthAPIError::UnexpectedError);

    // if cookie has issue generating return an error with the empty jar
    let auth_cookie= match auth_cookie{
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(e))
    };
    // If no error set the cookie in the jar
    let updated_jar = jar.add(auth_cookie);

    // return jar and regular auth variant of `LoginResponse`
    (updated_jar,Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}
