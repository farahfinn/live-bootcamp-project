
use std::{collections::HashMap, sync::Arc};

use auth_service::{app_state::AppState, domain::user::User, services::hashmap_user_store::HashmapUserStore, Application};
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store: HashMap<String, User> = HashMap::new();
        let app_state  = AppState{
            user_store: Arc::new(RwLock::new(
                HashmapUserStore{
                    users: user_store
                }))
        };
        let app = Application::build(app_state, "127.0.0.1:0" )
            .await
            .expect("Failed to build app");
        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task to avoid blocking
        // the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();// Create a reqwest http client instance
        // Create a new `TestApp` instance and return it
        Self{
            address,
            http_client
        } 
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    // TODO: Implement helper functions for all other routes (signup, login,
    // logout, verify-2fa and verify-token)

    pub async fn signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
     {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to post to signup route")
    }

    pub async fn login<B:serde::Serialize>(&self, body: &B) -> reqwest::Response {

        self.http_client
            .post(format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to post to login route")
        
    }

    pub async fn logout(&self) -> reqwest::Response {

        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to post to logout route")
        
    }

    pub async fn verify2fa<B: serde::Serialize>(&self, body: &B) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to post to verify2FA route")
        
    }
    pub async fn verifytoken<B: serde::Serialize> (&self, body: &B) -> reqwest::Response {

        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to post to verify-token route")
        
    }
}

pub fn get_random_email()-> String {
    format!("{}@example.com", Uuid::new_v4())
}

