use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::utils::auth::validate_token;

pub async fn verify_token(Json(request): Json<TokenRequest>) -> impl IntoResponse {
    let token = request.token;
    let validation_result = validate_token(&token).await;

    if validation_result.is_err(){
        StatusCode::UNAUTHORIZED.into_response()
    } else {
        StatusCode::OK.into_response() 
    }
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub token: String,
}
