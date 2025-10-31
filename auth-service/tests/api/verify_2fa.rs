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
    
    let app = TestApp::new().await;
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
}


#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

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
}
