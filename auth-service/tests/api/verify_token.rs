use serde_json::json;

use crate::helpers::TestApp;

#[tokio::test]
async fn verifytoken_returns_auth_ui() {
    let app = TestApp::new().await;
    let body = json!({
        "token": "some random token",
    });
    let response = app.verifytoken(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
