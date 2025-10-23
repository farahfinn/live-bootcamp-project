use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{domain::error::AuthAPIError, utils::{auth::validate_token, constants::JWT_COOKIE_NAME}};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>)  {
     // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken);

    match cookie {
        Ok(c)=> {
            let token = c.value().to_owned();
            let val_result= validate_token(&token).await;
            if val_result.is_err() {
                // return Invalid token 401, if cookie is not validated
                (jar, Err(AuthAPIError::InvalidToken))
       
            } else {
                let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));
               (jar, Ok(StatusCode::OK))
            }
            
        } ,
        Err(e) => (jar, Err(e))
        
    }



}
