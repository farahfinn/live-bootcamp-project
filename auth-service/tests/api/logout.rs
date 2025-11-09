use auth_service::{domain::data_store::BannedTokenStore, utils::constants::JWT_COOKIE_NAME};
use reqwest::{Url};
use serde_json::json;

use crate::helpers::TestApp;

// #[tokio::test]
// async fn logout_returns_auth_ui() {
//     let app = TestApp::new().await;
    
//     let response = app.logout().await;

//     assert_eq!(response.status().as_u16(), 200);
// }


#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let mut app = TestApp::new().await;

    // get a response without sending a cookie to the server
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 400);
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    //add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!("{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME),
         &Url::parse("http://127.0.0.1").expect("Failed to parse URL")
     );

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 401);
    // call clean up
    app.clean_up().await;

     
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
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
    let _response = app.login(&body1).await;
    // make sure that response from logout is valid as cookie was set during login
    let logout_res = app.logout().await;

    assert_eq!(logout_res.status().as_u16(), 200);
    // call clean up
    app.clean_up().await;

}

#[tokio::test]
async fn pass_if_jwt_is_added_to_banned_token_store() {
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
    // get the token that is set in the cookies when you login
    let token: String = response.cookies().find(|c| c.name() == JWT_COOKIE_NAME).expect("should find the cookie set up already").value().into();

    // logout to put the token in the banned store
    let _res2 = app.logout().await;
    {
        // New scope to borrow the app struct immutably and then drop it after in the outer scope

        //try using the token
        let banned_token_store = app.banned_token_store.read().await;

        // check if the token is in the store
        let res = banned_token_store.is_token_banned(token).await;

        assert!(res.is_ok(), "should be true");    
    }    // call clean up
    app.clean_up().await;

    
}


#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
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
    let _response = app.login(&body1).await;
    // call logout twice and verify response status 
    let _logout_res = app.logout().await;

    let logout_res1 = app.logout().await;

    assert_eq!(logout_res1.status().as_u16(), 400);
    // call clean up
    app.clean_up().await;

    
}
