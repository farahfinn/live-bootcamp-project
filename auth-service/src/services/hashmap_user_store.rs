use std::collections::HashMap;

use crate::domain::{data_store::{UserStore, UserStoreError}, email::Email, user::User};


#[derive(Default, Clone)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
   async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

   async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
       let email = Email::parse(email.into()).map_err(|_| UserStoreError::UserNotFound)?;
       match self.users.get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
       let email = Email::parse(email.into()).map_err(|_| UserStoreError::UserNotFound)?;
        if let Some(user) = self.users.get(&email) {
            match user.password.as_ref().cmp(password) {
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
    use crate::domain::password::Password;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let email = Email::parse("email@example.com".into()).unwrap();
        let password = Password::parse("password123".into()).unwrap();
        // Create an empty store
        let mut store = HashmapUserStore{users: HashMap::new()};
        // Create user
        let user = User::new(email.clone(), password, true);

        // test inserting user into store
        assert_eq!(store.add_user(user).await, Ok(()));
    }

    #[tokio::test]
   async fn test_get_user() {
        let email = Email::parse("email@example.com".into()).unwrap();
        let password = Password::parse("password123".into()).unwrap();
        // Create a user
        let user = User::new(email.clone(), password.clone(), true);

        // Put user in a the users store
        let users = HashMap::from([
            (email.clone(), user)
        ]);
        
        let store = HashmapUserStore{users};

        // check for user
        assert_eq!(store.get_user("email@example.com").await, Ok(User{email, password, requires_2fa: true }));
    }

    #[tokio::test]
   async fn test_validate_user() {
        let email = Email::parse("email@example.com".into()).unwrap();
        let password = Password::parse("password123".into()).unwrap();
        // create user
        let user = User::new(email.clone(), password.clone(), true);

        // create a users store Hashmap with user
        let users = HashMap::from([
            (email.clone(), user)
        ]);
        // inser users into the hashmap store
        let store = HashmapUserStore{users};

        assert_eq!(store.validate_user(email.as_ref(), password.as_ref()).await, Ok(()));
    }
}
