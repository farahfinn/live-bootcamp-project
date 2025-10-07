
use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.signup().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.login().await;

    assert_eq!(response.status().as_u16(), 200);
}
#[tokio::test]
async fn logout_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}
#[tokio::test]
async fn verify2fa_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.verify2fa().await;

    assert_eq!(response.status().as_u16(), 200);
}
#[tokio::test]
async fn verifytoken_returns_auth_ui() {
    let app = TestApp::new().await;
    
    let response = app.verifytoken().await;

    assert_eq!(response.status().as_u16(), 200);
}
