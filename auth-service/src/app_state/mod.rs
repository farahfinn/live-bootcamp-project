use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{domain::data_store::UserStore, services::hashmap_user_store::HashmapUserStore};

// pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

#[derive(Clone)]
pub struct AppState<T: UserStore> {
    pub user_store: Arc<RwLock<T>>,
}

impl<T: UserStore> AppState<T> {
    pub fn new(user_store: Arc<RwLock<T>>) -> Self {
        Self { user_store }
    }
}
