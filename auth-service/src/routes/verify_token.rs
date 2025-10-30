use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, services::{hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore}, utils::auth::validate_token};

pub async fn verify_token(State(AppState {banned_token_store, .. }): State<AppState<HashmapUserStore, HashsetBannedTokenStore, HashmapTwoFACodeStore>>,Json(request): Json<TokenRequest>) -> impl IntoResponse {
    let token = request.token;

    let banned_store = banned_token_store.read().await;

    if banned_store.0.contains(&token) {
        StatusCode::UNAUTHORIZED.into_response()
    } else {
        let validation_result = validate_token(&token).await;

        if validation_result.is_err(){
            StatusCode::UNAUTHORIZED.into_response()
        } else {
            StatusCode::OK.into_response() 
        }
    } 
    
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub token: String,
}
