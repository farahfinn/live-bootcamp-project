use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;

use crate::helpers::TestApp;

// #[tokio::test]
// async fn logout_returns_auth_ui() {
//     let app = TestApp::new().await;
    
//     let response = app.logout().await;

//     assert_eq!(response.status().as_u16(), 200);
// }


#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    // get a response without sending a cookie to the server
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    //add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!("{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME),
         &Url::parse("http://127.0.0.1").expect("Failed to parse URL")
     );

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 401);
     
}
