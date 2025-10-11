use serde_json::json;

use crate::helpers::TestApp;

#[tokio::test]
async fn verify2fa_returns_auth_ui() {
    let app = TestApp::new().await;
    let body = json!({
        "email": "example@email.com",
        "LoginAttemptId": "random no",
        "2FACode": "jaou5241kjocsx",
    });
    let response = app.verify2fa(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
