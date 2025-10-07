use std::collections::HashMap;

use auth_service::Application;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
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

    pub async fn signup(&self) -> reqwest::Response {
        let mut params = HashMap::new();

        params.insert("email", "johndoe@example.com");
        params.insert("password", "password123");
        params.insert("requires2FA", "True");       
        self.http_client
            .post(format!("{}/signup", &self.address))
            .form(&params)
            .send()
            .await
            .expect("Failed to post to signup route")
    }

    pub async fn login(&self) -> reqwest::Response {
        let mut params = HashMap::new();

        params.insert("email", "johndoe@example.com");
        params.insert("password", "password123");
        self.http_client
            .post(format!("{}/login", &self.address))
            .form(&params)
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

    pub async fn verify2fa(&self) -> reqwest::Response {
        let mut params = HashMap::new();

        params.insert("email", "johndoe@example.com");
        params.insert("loginAttemptID", "123");
        params.insert("2FACode", "123");
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .form(&params)
            .send()
            .await
            .expect("Failed to post to verify2FA route")
        
    }
    pub async fn verifytoken(&self) -> reqwest::Response {
        let mut params = HashMap::new();

        params.insert("token", "example token");
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .form(&params)
            .send()
            .await
            .expect("Failed to post to verify-token route")
        
    }
}

