//This struct encapsulates our application-related logic

use std::error::Error;

use axum::{http::{Method, StatusCode}, response::IntoResponse, routing::post, serve::Serve, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, services::ServeDir};
use crate::{app_state::AppState, domain::error::AuthAPIError, routes::{login, logout, signup, verify2fa, verify_token }, services::{hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore, mock_email_client::MockEmailClient}};


pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;

pub struct Application {
    server: Serve<Router, Router>,
    // adress is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState<HashmapUserStore, HashsetBannedTokenStore, HashmapTwoFACodeStore, MockEmailClient>, address: &str ) -> Result<Self, Box<dyn Error>> {
        // Allow the app service (running on our local machine & in production) to call the auth service
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            // TODO when deploying
            // http://DROPLETIP:8000
        ];
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance & return it
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}


#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials=> (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "User does not exist"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token")
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}
