use auth_service::{domain::{data_store::TwoFACodeStore, email::Email}, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME};
use serde_json::json;

use crate::helpers::{get_random_email, TestApp};


#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let mut app = TestApp::new().await;
    let body = json!({
        "email": "example@email.com"
    });

    // should get an error if incorrect body
    let response = app.login(&body).await;

    assert_eq!(response.status().as_u16(), 422);
    // call clean up
    app.clean_up().await;

}


#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;

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
    // call clean up
    app.clean_up().await;

}


#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.     
    let mut app = TestApp::new().await;
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
    // call clean up
    app.clean_up().await;

    
}


 #[tokio::test]
async fn should_return_200_if_valid_creds_and_2fa_disabled() {
    let mut app = TestApp::new().await;
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
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();
    let signup_body = json!({
        "email": random_email,
        "password": "Password123",
        "requires2FA": true
    });
    let response = app.signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = json!({
        "email": random_email,
        "password": "Password123",
    });

    let response = app.login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let response_json = response
                .json::<TwoFactorAuthResponse>()
                .await
                .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert_eq!(response_json.message, "2FA required".to_owned());

    // get id from the response
    let login_attempt_id = response_json.login_attempt_id;

    {
        // Needed new scope so that I can borrow the app to get code store while having a mutable borrow later
        // to drop.
        
        // code store with the two_fa_codes
        let codes_store = app.two_fa_code_store.read().await;
        let email = Email::parse(random_email).expect("email should be parsed ok");
        let (login_attempt_id_from_app, _two_fa_code_from_app) = codes_store
            .get_code(&email)
            .await
            .expect("should find the code ");
        assert_eq!(&login_attempt_id, login_attempt_id_from_app.as_ref());
    }
    // call clean up
    app.clean_up().await;

}
