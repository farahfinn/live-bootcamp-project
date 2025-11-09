
use auth_service::{domain::email::Email, utils::{auth::generate_auth_cookie, constants::JWT_COOKIE_NAME}};
use serde_json::json;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_200_if_valid_token() {
    let mut app = TestApp::new().await;

    let body = json!({
        "email": "example@email.com",
        "password": "password123",
        "verify2FA": true,
    });

    let _response = app.signup(&body).await;
    let body = json!({
        "email": "example@email.com",
        "password": "password123",
    });
    let _response = app.login(&body).await;

    let cookie= generate_auth_cookie(Email::parse("example@email.com".to_string()).unwrap()).unwrap();

    let token = cookie.value();

    let body = json!({
        "token": token,
    });
    let response = app.verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 200);
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let empty_token = json!({
        "malformed": "",
    });
    let no_token_in_body = json!({});

    let bodies = [empty_token, no_token_in_body];

    for body in &bodies {
        let response = app.verify_token(body).await;
        
        assert_eq!(response.status().as_u16(), 422);
    }
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    let body = json!({
        "email": "example@email.com",
        "password": "password123",
        "verify2FA": true,
    });

    let _response = app.signup(&body).await;
    let body = json!({
        "email": "example@email.com",
        "password": "password123",
    });
    let _response = app.login(&body).await;



    let body = json!({
        "token": "random wrong token",
    });
    let response = app.verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 401);

    
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let mut app = TestApp::new().await;

    
    let body = json!({
        "email": "example@email.com",
        "password": "Password123",
        "requires2FA": false,
    });
    let body1 = json!({
        "email": "example@email.com",
        "password": "Password123",
    });
    // signup user
    let _res1 = app.signup(&body).await;
    // login and get a cookie back from server
    let response = app.login(&body1).await;
    // get the token from the cookie name that is set when you login
    let token = response
        .cookies()
        .find(|c|  c.name() == JWT_COOKIE_NAME )
        .expect("cookie should exist")
        .value()
        .parse::<String>()
        .expect("should be able to convert str to String");

    // move the cookie to banned store by logging out
    let _response = app.logout().await;

    // verify token should then return 401
    let verify_body = json!({
        "token": token,
    });
    let response = app.verify_token(&verify_body).await;

    assert_eq!(response.status().as_u16(), 401); 
    // call clean up
    app.clean_up().await;

}
