use auth_service::utils::constants::JWT_COOKIE_NAME;
use serde_json::json;

use crate::helpers::{get_random_email, TestApp};


#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let body = json!({
        "email": "example@email.com"
    });

    // should get an error if incorrect body
    let response = app.login(&body).await;

    assert_eq!(response.status().as_u16(), 422);
}


#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    // body of json has invalid input
    let body = json!({
        "email": "noatsymbol.com",
        "password": "goodpassword1223"
    });

    let body2 = json!({
        "email": "goodemail@email.com",
        "password": "badpass"
    });
    let response = app.login(&body).await;
    let response2 = app.login(&body2).await;

    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(response2.status().as_u16(), 400);
}


#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.     
    let app = TestApp::new().await;
    let body = json!({
        "email": "example@email.com",
        "password": "Password123",
        "requires2FA": true
    });
    let body2  = json!({
        "email": "nonexistenuser@email.com",
        "password": "password123"
    });

    // create a user in the store
    app.signup(&body).await;

    let response2 = app.login(&body2).await;

    assert_eq!(response2.status().as_u16(), 401);
    
}


 #[tokio::test]
async fn should_return_200_if_valid_creds_and_2fa_disabled() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    let signup_body = json!({
        "email": random_email,
        "password": "Password123",
        "requires2FA": false
    });
    let response = app.signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = json!({
        "email": random_email,
        "password": "Password123",
    });

    let response = app.login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}
