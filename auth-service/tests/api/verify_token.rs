use crate::helpers::TestApp;

#[tokio::test]
async fn verifytoken_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.verifytoken().await;

    assert_eq!(response.status().as_u16(), 200);
}
