use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn verify2fa(Json(request): Json<VerifyRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    #[serde(rename = "LoginAttemptId")]
    pub loginattemptid: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
