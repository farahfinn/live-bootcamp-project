use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::data_store::{BannedTokenStore, UserStore};

// pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

#[derive(Clone)]
pub struct AppState<T: UserStore, U: BannedTokenStore> {
    pub user_store: Arc<RwLock<T>>,
    pub banned_token_store: Arc<RwLock<U>>,
}

impl<T: UserStore, U: BannedTokenStore> AppState<T, U> {
    pub fn new(user_store: Arc<RwLock<T>>, token_store: Arc<RwLock<U>>) -> Self {
        Self {
            user_store,
            banned_token_store: token_store,
        }
    }
}
