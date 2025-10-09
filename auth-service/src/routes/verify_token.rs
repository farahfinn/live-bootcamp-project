use axum::{http::StatusCode, response::IntoResponse};

pub async fn verifytoken() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
