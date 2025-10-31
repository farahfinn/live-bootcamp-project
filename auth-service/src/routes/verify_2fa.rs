use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::domain::{data_store::{LoginAttemptId, TwoFACode}, email::Email};

pub async fn verify2fa(Json(request): Json<VerifyRequest>) -> impl IntoResponse {
    let email = match Email::parse(request.email) {
        Ok(e) => e,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    let login_attempt_id = match LoginAttemptId::parse(request.loginattemptid) {
        Ok(id) => id,
        Err(_) => return StatusCode::BAD_REQUEST.into_response()
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(code) => code,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
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
