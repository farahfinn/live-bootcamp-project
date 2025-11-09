use std::error::Error;

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::PgPool;

use crate::domain::{data_store::{UserStore, UserStoreError}, email::Email, password::Password, user::User};
#[derive(Clone )]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
   async fn add_user(&mut self, user: User) -> Result<(), UserStoreError > {
        println!("Adding user to the database...");
        let email = user.email.as_ref();
        let password_hash = compute_password_hash(user.password.as_ref()).await.map_err(|_| UserStoreError::UnexpectedError)?;
        let requires_2fa  = user.requires_2fa;
        let result = sqlx::query!(
            r#"INSERT INTO users (email, password_hash, requires_2fa ) VALUES ($1, $2, $3)"#,
            email, password_hash, requires_2fa
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                println!("User added successfully.");
                Ok(())
            }
            Err(e) => {
                println!("Error adding user: {:?}", e);
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        return Err(UserStoreError::UserAlreadyExists);
                    }
                }
                Err(UserStoreError::UnexpectedError)
            }
        }
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        println!("Searching for user with email: {}", email);
        let user_record = sqlx::query!(
            r#"
                SELECT email, password_hash, requires_2fa
                FROM users
                WHERE email = $1
            "#,
            email.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        if let Some(record) = user_record {
            let email = Email::parse(record.email).map_err(|_| UserStoreError::UnexpectedError)?;
            let password = Password::parse(record.password_hash).map_err(|_| UserStoreError::UnexpectedError)?;
             Ok(
                 User{
                     email,
                     password,
                     requires_2fa: record.requires_2fa
                 })
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result< (), UserStoreError> {
        let email = Email::parse(email.into()).map_err(|_| UserStoreError::UnexpectedError)?;
        if let Ok(user) = self.get_user(email.as_ref()).await {
            match verify_password_hash(user.password.as_ref(), password).await {
                Ok(()) => Ok(()),
                Err(_) => Err(UserStoreError::InvalidCredentials),
            }
        } else {
            Err(UserStoreError::UnexpectedError)
        }
    }
}

async fn verify_password_hash(expected_pass_hash:&str, password_candidate: &str) -> Result<(), Box<dyn Error>> {
    let expected_pass_hash = expected_pass_hash.to_string();
    let password_candidate = password_candidate.to_string();

    tokio::task::spawn_blocking(move ||{
        // First, parse the stored hash string
        let parsed_hash = PasswordHash::new(&expected_pass_hash)?;
        // Then verify the candidate password against the parsed hash
        Argon2::default().verify_password(password_candidate.as_bytes(), &parsed_hash)
    } ).await??; // The first "?" unwraps the JoinError, the second unwraps the password_hash::Error
    Ok(())
}

async fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error>> {
    let password = password.to_string();
    let password_hash = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::new(1500, 2, 1, None).unwrap(),
        );
        // Hash the password and immediately convert it to an owned String
        // to avoid lifetime issues with the salt.
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string()) // Convert the PasswordHash<'a> to a String
    })
    .await??; // The first '?' handles the JoinError, the second handles the hashing Error
    Ok(password_hash)
}
