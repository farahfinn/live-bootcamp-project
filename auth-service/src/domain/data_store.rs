use crate::domain::user::User;



#[async_trait::async_trait]
pub trait UserStore  {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError > ;
        
    async fn get_user(&self, email: &str) -> Result<User, UserStoreError > ;
    
    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError > ;
}
#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn store_token(&mut self,token: String)-> Result<(), BannedTokenStoreError>; 
    
    async fn is_token_banned(&self, token: String) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpecteError,
}

#[derive(Debug,PartialEq)]
pub enum BannedTokenStoreError {
    TokenAlreadyInStore
}
