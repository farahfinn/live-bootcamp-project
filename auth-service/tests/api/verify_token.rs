use auth_service::{domain::email::Email, utils::auth::generate_auth_cookie};
use serde_json::json;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_200_if_valid_token() {
    let app = TestApp::new().await;

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
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let empty_token = json!({
        "malformed": "",
    });
    let no_token_in_body = json!({});

    let bodies = [empty_token, no_token_in_body];

    for body in &bodies {
        let response = app.verify_token(body).await;
        
        assert_eq!(response.status().as_u16(), 422);
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

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

    
}
