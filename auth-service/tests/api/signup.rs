use auth_service::routes::SignupResponse;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn signup_returns_201_if_valid_input() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let body = serde_json::json!({
        "email": email ,
        "password" : "randompassword13k",
        "requires2FA": true,
    });
    
    let response = app.signup(&body).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );

    
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": false
        }),
        serde_json::json!({
            "email": random_email,
            "password": "somepassword25"
        })
    ];

    for test_case in test_cases.iter() {
        let response = app.signup(test_case).await;
        // assert with custom panic message
        assert_eq!(response.status().as_u16(), 422, "Failed to input: {:?}", test_case);
    }
}

