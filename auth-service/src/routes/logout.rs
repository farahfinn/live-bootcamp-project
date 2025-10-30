use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{app_state::AppState, domain::{data_store::BannedTokenStore, error::AuthAPIError}, services::{hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore}, utils::{auth::validate_token, constants::JWT_COOKIE_NAME}};

pub async fn logout(State(state): State<AppState<HashmapUserStore, HashsetBannedTokenStore, HashmapTwoFACodeStore>> ,jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>)  {
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
                // add the token to the banned token store
                let mut banned_store = state.banned_token_store.write().await;
                let res = banned_store.store_token(token).await;
                if res.is_err() {
                    (jar, Err(AuthAPIError::UnexpectedError))
                }else {
                    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));
                   (jar, Ok(StatusCode::OK))
                    
                }
            }
            
        } ,
        Err(e) => (jar, Err(e))
        
    }



}
