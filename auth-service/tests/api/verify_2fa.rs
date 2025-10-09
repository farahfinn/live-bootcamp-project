use crate::helpers::TestApp;

#[tokio::test]
async fn verify2fa_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.verify2fa().await;

    assert_eq!(response.status().as_u16(), 200);
}
