use serde_json::json;

use crate::helpers::TestApp;

 #[tokio::test]
async fn login_returns_auth_ui() {
    let app = TestApp::new().await;
    let body = json!({
        "email": "example@email.com",
        "password": "Password123"
    });
    let response = app.login(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
