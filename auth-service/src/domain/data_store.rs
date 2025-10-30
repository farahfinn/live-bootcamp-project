
use rand::Rng;

use crate::domain::{email::Email, user::User};



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
// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore {
     async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
     async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
     async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        // Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
        let uuid= uuid::Uuid::parse_str(&id);

        match uuid {
            Ok(uuid)=> Ok(Self(uuid.into())),
            Err(_) => Err("Failed to parse UUID".into()) 
        }

    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        let uuid = uuid::Uuid::new_v4();
        Self(uuid.into())
    }
}

//Implement AsRef<str> for LoginAttemptId
impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        // Ensure `code` is a valid 6-digit code
        if code.len() != 6 {
            Err("Code is not 6 digits".into())
        } else {
            Ok(Self(code))
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FA code.
        // The code should be 6 digits (ex: 834629)
        let mut rng = rand::rng();
        
        let random_6_digits = rng.random_range(100_000..=999_999);
        let random_6_digits = format!("{}",random_6_digits);
        Self(random_6_digits)
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
