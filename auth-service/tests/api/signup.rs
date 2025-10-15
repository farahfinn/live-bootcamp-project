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

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters
    let invalid_inputs = [
        serde_json::json!({
            "email" : "",
            "password": "password132",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email" : "noatsymbol.com",
            "password": "password132",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email" : "example@email.com",
            "password": "short",
            "requires2FA": true,
        })
    ];

    for input in invalid_inputs.iter(){
        let response = app.signup(input).await;

        assert_eq!(response.status().as_u16(), 400);
    }
}


#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
 
    let input = serde_json::json!({
         "email": "example@email.com",
         "password": "validpassword1235",
         "requires2FA": true,
    });
    // Call the signup route twice. The second request should fail with a 409 HTTP status code    
    app.signup(&input).await;
    let response = app.signup(&input).await;

    assert_eq!(response.status().as_u16(), 409);
}
