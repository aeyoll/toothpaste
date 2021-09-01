use futures::lock::Mutex;
use rbatis::rbatis::Rbatis;
use std::sync::Arc;

pub type DatabasePool = Arc<Mutex<Rbatis>>;

// Initializes the cache, using the given configuration.
pub async fn create_database_pool() -> DatabasePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let rb = Rbatis::new();
    rb.link(&database_url).await.unwrap();

    Arc::new(Mutex::new(rb))
}
