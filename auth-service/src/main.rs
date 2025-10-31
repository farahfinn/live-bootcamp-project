use std::{collections::{HashMap, HashSet}, sync::Arc};

use auth_service::{app_state::AppState, domain::{data_store::{LoginAttemptId, TwoFACode}, email::Email, user::User}, services::{hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore, mock_email_client::MockEmailClient}, utils::constants::prod, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {

    let user_store: HashMap<Email, User> = HashMap::new();
    let banned_token_store: HashSet<String> = HashSet::new();
    let two_fa_code_store: HashMap<Email, (LoginAttemptId, TwoFACode)> = HashMap::new();
    let email_client = MockEmailClient;
    let app_state  = AppState::new(
        Arc::new(RwLock::new(HashmapUserStore{users:user_store})),
        Arc::new(RwLock::new(HashsetBannedTokenStore(banned_token_store))),
        Arc::new(RwLock::new(HashmapTwoFACodeStore{codes: two_fa_code_store})),
        Arc::new(RwLock::new(email_client)));
           
    let app = Application::build(app_state,prod::APP_ADDRESS).await.expect("Failed to build app");

    app.run().await.expect("Failed to run app")
}


