//This struct encapsulates our application-related logic

use std::error::Error;

use axum::{response::Html, routing::{get, post}, serve::Serve, Router};
use tower_http::services::ServeDir;
use crate::routes::{login, logout, signup, verify2fa, verifytoken};
pub mod routes;

pub struct Application {
    server: Serve<Router, Router>,
    // adress is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
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
            .route("/verify-token", post(verifytoken));

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


