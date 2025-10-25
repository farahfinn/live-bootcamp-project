use std::collections::HashSet;

use crate::domain::data_store::{BannedTokenStore, BannedTokenStoreError};
#[derive(Debug, Clone)]
pub struct HashsetBannedTokenStore(pub HashSet<String>);

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store_token(&mut self, token: String)-> Result<(), BannedTokenStoreError> {
       let insert_result = self.0.insert(token);
       if insert_result {
           Ok(())
       } else {
           Err(BannedTokenStoreError::TokenAlreadyInStore)
       }
    }

    async fn is_token_banned(&self, token:String) -> bool{
         self.0.contains(&token)
    }
}


#[cfg(test)]
mod tests {
    use crate::{domain::email::Email, utils::auth::generate_auth_cookie};

    use super::*;

    #[tokio::test]
    async fn test_store_token() {
        // create a banned token store
        let mut store = HashsetBannedTokenStore(HashSet::new());

        // create a token from email
        let email = Email::parse("example@email.com".to_string()).expect("Should parse the email");

        let cookie = generate_auth_cookie(email).expect("shoudl get a cookie");

        let token = cookie.value();
        // store the token in the store
        let result = store.store_token(token.into()).await;
        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn test_is_token_banned() {
        // create a banned token store
        let mut store = HashsetBannedTokenStore(HashSet::new());

        // create a token from email
        let email = Email::parse("example@email.com".to_string()).expect("Should parse the email");
        let email1 = Email::parse("example2@email.com".to_string()).expect("Should parse the email succesfully");
        
        let cookie = generate_auth_cookie(email).expect("shoudl get a cookie");
        let cookie2 = generate_auth_cookie(email1).expect("should create cookie");

        let token = cookie.value();
        let token2 = cookie2.value();
        // store the token in the store
         store.store_token(token.into()).await.expect("Should store the token successfully");

        // check if token is banned
        let result = store.is_token_banned(token.into()).await;
        let result2 = store.is_token_banned(token2.into()).await;
        assert!(result);
        assert!(!result2, "This should be false ");
        
    }
}
