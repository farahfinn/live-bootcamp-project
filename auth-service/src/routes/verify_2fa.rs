use axum::{http::StatusCode, response::IntoResponse};

pub async fn verify2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

