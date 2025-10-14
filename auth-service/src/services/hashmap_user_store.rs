use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpecteError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            match user.password.as_str().cmp(password) {
                std::cmp::Ordering::Equal => Ok(()),
                _ => Err(UserStoreError::InvalidCredentials),
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let email = String::from("email@example.com");
        let password = String::from("password123");
        let user = User::new(email.clone(), password, true);
        let users = HashMap::from([
            (email, user)
        ]);
        
        let mut store = HashmapUserStore{users};

        let user2 = User::new("email2@example.com".into(), "safepassword".into(), true);
        assert_eq!(store.add_user(user2), Ok(()));
    }

    #[tokio::test]
   async fn test_get_user() {
        let email = String::from("email@example.com");
        let password = String::from("password123");
        let user = User::new(email.clone(), password.clone(), true);
        let users = HashMap::from([
            (email.clone(), user)
        ]);
        
        let store = HashmapUserStore{users};
        assert_eq!(store.get_user("email@example.com"), Ok(User{email, password, requires_2fa: true }));
    }

    #[tokio::test]
   async fn test_validate_user() {
        let email = String::from("email@example.com");
        let password = String::from("password123");
        let user = User::new(email.clone(), password.clone(), true);
        let users = HashMap::from([
            (email.clone(), user)
        ]);
        
        let store = HashmapUserStore{users};

        assert_eq!(store.validate_user(&email, &password), Ok(()));
    }
}
