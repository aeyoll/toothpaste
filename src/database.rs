use sea_orm::{Database, DatabaseConnection};

// Initializes the database pool
pub async fn create_database_pool() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    db
}
