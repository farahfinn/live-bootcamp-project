use auth_service::{domain::{data_store::TwoFACodeStore, email::Email}, utils::constants::JWT_COOKIE_NAME};
use serde_json::json;
use uuid::Uuid;

use crate::helpers::TestApp;

// #[tokio::test]
// async fn verify2fa_returns_auth_ui() {
//     let app = TestApp::new().await;
//     let body = json!({
//         "email": "example@email.com",
//         "LoginAttemptId": "random no",
//         "2FACode": "jaou5241kjocsx",
//     });
//     let response = app.verify2fa(&body).await;

//     assert_eq!(response.status().as_u16(), 200);
// }

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    
    let mut app = TestApp::new().await;
    let malformed_body = [
        json!({
        "email": "example@email.com",
        "2FACode": "123456",
    }),
       json!({
        "email": "example@email.com",
        "LoginAttemptId": "random no",
    }),
       json!({
        "LoginAttemptId": "random no",
        "2FACode": "123456",
    })];
    for body in malformed_body{
        let response = app.verify2fa(&body).await;
        assert_eq!(response.status().as_u16(), 422);
    }
    // call clean up
    app.clean_up().await;

}


#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;

    let valid_uuid: String = Uuid::new_v4().into();
    let invalid_body = [
        json!({
            "email": "wrongemailnoat.com",
            "LoginAttemptId": valid_uuid,
            "2FACode": "123456",
        }),
        // invalid 2FACode
        json!({
            "email": "example@email.com",
            "LoginAttemptId": valid_uuid,
            "2FACode": "1234",
        }),
        json!({
            "email": "example@email.com",
            "LoginAttemptId": valid_uuid,
            "2FACode": "123jkl",
        }),
        json!({
            "email": "example@email.com",
            "LoginAttemptId": "InvalidUUID",
            "2FACode": "1234",
        })
    ];

    for body in invalid_body {
        let response = app.verify2fa(&body).await;
        assert_eq!(response.status().as_u16(), 400);
    }
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let mut app = TestApp::new().await;

    let signup_body = json!({
       "email": "example@email.com",
       "password": "Password123",
       "requires2FA": true, 
    });

    let _res = app.signup(&signup_body).await;

    let login_body = json!({
       "email": "example@email.com",
       "password": "Password123",
    });
    let _res2 = app.login(&login_body).await;
    let email = Email::parse("example@email.com".into()).expect("an email should be parsed ");
    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .expect("login id and 2FA code should be set");

    let correct_login_attempt_id = login_attempt_id.as_ref().to_string();
    let correct_two_fa_code = two_fa_code.as_ref().to_string();
    let invalid_uuid = Uuid::new_v4();
    let invalid_two_fa_code = "123898".to_string();
    let verify_body = [
        json!({
        "email": "example@email.com",
        "LoginAttemptId": invalid_uuid,
        "2FACode": correct_two_fa_code,
    }),
        json!({
        "email": "example@email.com",
        "LoginAttemptId": correct_login_attempt_id,
        "2FACode": invalid_two_fa_code,
        })
    ];

    for body in verify_body {
        let response = app.verify2fa(&body).await;

        assert_eq!(response.status().as_u16(), 401);
    }
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let mut app = TestApp::new().await;

    let signup_body = json!({
       "email": "example@email.com",
       "password": "Password123",
       "requires2FA": true, 
    });

    let _res = app.signup(&signup_body).await;

    let login_body = json!({
       "email": "example@email.com",
       "password": "Password123",
    });
    let _res2 = app.login(&login_body).await;
    let email = Email::parse("example@email.com".into()).expect("an email should be parsed ");
    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .expect("login id and 2FA code should be set");

    let old_login_attempt_id = login_attempt_id.as_ref().to_string();
    let old_two_fa_code = two_fa_code.as_ref().to_string();

    // call login again to get new id and 2FA code
    let _res = app.login(&login_body).await;
    
    let verify_body = json!({
        "email": "example@email.com",
        "LoginAttemptId": old_login_attempt_id,
        "2FACode": old_two_fa_code,
    });

    let response = app.verify2fa(&verify_body).await;

    assert_eq!(response.status().as_u16(), 401);
    
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let mut app = TestApp::new().await;

    let signup_body = json!({
       "email": "example@email.com",
       "password": "Password123",
       "requires2FA": true, 
    });

    let _res = app.signup(&signup_body).await;

    let login_body = json!({
       "email": "example@email.com",
       "password": "Password123",
    });
    let _res2 = app.login(&login_body).await;
    let email = Email::parse("example@email.com".into()).expect("an email should be parsed ");
    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .expect("login id and 2FA code should be set");

    let correct_login_attempt_id = login_attempt_id.as_ref().to_string();
    let correct_two_fa_code = two_fa_code.as_ref().to_string();
    let verify_body = json!({
        "email": "example@email.com",
        "LoginAttemptId": correct_login_attempt_id,
        "2FACode": correct_two_fa_code,
    });

    let response = app.verify2fa(&verify_body).await;

    assert_eq!(response.status().as_u16(), 200);

    // ensure that an auth cookie is set on login
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("a cookie name jwt should be set");

    assert!(!auth_cookie.value().is_empty());
    
    
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let mut app = TestApp::new().await;

    let signup_body = json!({
       "email": "example@email.com",
       "password": "Password123",
       "requires2FA": true, 
    });

    let _res = app.signup(&signup_body).await;

    let login_body = json!({
       "email": "example@email.com",
       "password": "Password123",
    });
    let _res2 = app.login(&login_body).await;
    let email = Email::parse("example@email.com".into()).expect("an email should be parsed ");
    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .expect("login id and 2FA code should be set");

    let correct_login_attempt_id = login_attempt_id.as_ref().to_string();
    let correct_two_fa_code = two_fa_code.as_ref().to_string();
    let verify_body = json!({
        "email": "example@email.com",
        "LoginAttemptId": correct_login_attempt_id,
        "2FACode": correct_two_fa_code,
    });

    let _response = app.verify2fa(&verify_body).await;
    // sending a request to verify2fa twice should not work
    let response = app.verify2fa(&verify_body).await;

    assert_eq!(response.status().as_u16(), 401);
    // call clean up
    app.clean_up().await;

}
