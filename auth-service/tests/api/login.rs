use crate::helpers::TestApp;

 #[tokio::test]
async fn login_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.login().await;

    assert_eq!(response.status().as_u16(), 200);
}
