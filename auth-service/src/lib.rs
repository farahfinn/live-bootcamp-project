//This struct encapsulates our application-related logic

use std::error::Error;

use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, serve::Serve, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use crate::{app_state::AppState, domain::error::AuthAPIError, routes::{login, logout, signup, verify2fa, verifytoken}, services::hashmap_user_store::HashmapUserStore};


pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;


pub struct Application {
    server: Serve<Router, Router>,
    // adress is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState<HashmapUserStore>, address: &str ) -> Result<Self, Box<dyn Error>> {
        // Move the Router definiton from `main.rs` to here.
        // Also, remover the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/hello", get(hello_handler))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify2fa))
            .route("/verify-token", post(verifytoken))
            .with_state(app_state);

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
async fn hello_handler() -> Html<&'static str> {
    // TODO: Update this to a custom message!
    Html("<h1>Hello from sprint 1</h1>")
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
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "User does not exist")
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}
