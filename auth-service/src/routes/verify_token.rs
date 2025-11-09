use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::data_store::BannedTokenStore, services::{data_store::PostgresUserStore, mock_email_client::MockEmailClient, redis_banned_token_store::RedisBannedTokenStore, redis_two_fa_code_store::RedisTwoFACodeStore}, utils::auth::validate_token};

pub async fn verify_token(State(AppState {banned_token_store, .. }): State<AppState<PostgresUserStore, RedisBannedTokenStore, RedisTwoFACodeStore, MockEmailClient>>,Json(request): Json<TokenRequest>) -> impl IntoResponse {
    let token = request.token;

    let banned_store = banned_token_store.read().await;

    match banned_store.is_token_banned(token.clone()).await {
        Ok(true) => {
            // the token is banned
            StatusCode::UNAUTHORIZED.into_response()
        },
        Ok(false) => {
            // Token is not banned, proceed with the validation
            let validation_result = validate_token(&token).await;

            if validation_result.is_err(){
                StatusCode::UNAUTHORIZED.into_response()
            } else {
                StatusCode::OK.into_response() 
            }
        }
        Err(_) => {
            StatusCode::UNAUTHORIZED.into_response()
        }
    }

    
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub token: String,
}
