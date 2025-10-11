use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;



pub async fn login(Json(request): Json<LoginRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

