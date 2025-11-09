use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{domain::data_store::{BannedTokenStore, BannedTokenStoreError}, utils::auth::TOKEN_TTL_SECDONDS};

#[derive(Clone)]
pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn store_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        // create a new key using helper fn
        let key =  get_key(&token);
        // call the set_ex command on the Redis connection to se ta new key/value pair
        // with an expiration time (TTL)
        let ttl: u64= TOKEN_TTL_SECDONDS.try_into().map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        let mut connection = self
            .conn
            .write()
            .await;
        connection
            .set_ex(key, true, ttl)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        Ok(())
        
    }
    async fn is_token_banned(&self, token:String) -> Result<bool, BannedTokenStoreError> {
        let mut connection = self
            .conn
            .write()
            .await;

        // add the prefix to the token to before searching
        let token = get_key(&token);
        let check_result:Result<bool, BannedTokenStoreError> = connection.exists(token)
            .map_err(|_| BannedTokenStoreError::UnexpectedError);

        check_result
    }
}
// we are suing a key prefix to prevent collisons and organize data
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";
fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
