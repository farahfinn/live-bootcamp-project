use std::sync::Arc;

use auth_service::{app_state::AppState, get_postgres_pool, get_redis_client, services::{data_store::PostgresUserStore, mock_email_client::MockEmailClient, redis_banned_token_store::RedisBannedTokenStore, redis_two_fa_code_store::RedisTwoFACodeStore}, utils::constants::{prod, DATABASE_URL, REDIS_HOST_NAME}, Application};
use sqlx::PgPool;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgres().await ;
    let database_store = PostgresUserStore::new(pg_pool);
    // let _user_store: HashMap<Email, User> = HashMap::new();
    
    // Use helper fn to get the redis connection
    let conn = configure_redis();
    let conn = Arc::new(RwLock::new(conn));
    // The conn field in the BannedRedisStore has a Rwlock
    let banned_token_store = RedisBannedTokenStore::new(conn.clone()); 
    // let banned_token_store: HashSet<String> = HashSet::new();
    let two_fa_code_store = RedisTwoFACodeStore::new(conn);
    let email_client = MockEmailClient;
    let app_state  = AppState::new(
        Arc::new(RwLock::new(database_store)),
        Arc::new(RwLock::new(banned_token_store)),
        Arc::new(RwLock::new(two_fa_code_store)),
        Arc::new(RwLock::new(email_client)));
           
    let app = Application::build(app_state,prod::APP_ADDRESS).await.expect("Failed to build app");

    app.run().await.expect("Failed to run app")
}


async fn configure_postgres() -> PgPool {
    // create anew database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL).
        await
        .expect("Failed to create Postgres connection pool");
    // Run database migration against our test database
    sqlx::migrate!().run(&pg_pool).await.expect("Failed to run migrations");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_string())
        .expect("Failed to get Redis Client")
        .get_connection()
        .expect("Failed to get Redis Connection")
}
