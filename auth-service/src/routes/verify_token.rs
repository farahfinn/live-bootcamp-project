use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn verifytoken(Json(request): Json<TokenRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub token: String,
}
