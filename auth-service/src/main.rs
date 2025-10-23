use std::{collections::HashMap, sync::Arc};

use auth_service::{app_state::AppState, domain::{email::Email, user::User}, services::hashmap_user_store::HashmapUserStore, utils::constants::prod, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {

    let user_store: HashMap<Email, User> = HashMap::new();
    let app_state  = AppState::new(Arc::new(RwLock::new(HashmapUserStore{users:user_store})));
    let app = Application::build(app_state,prod::APP_ADDRESS).await.expect("Failed to build app");

    app.run().await.expect("Failed to run app")
}


