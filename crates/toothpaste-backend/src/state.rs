use sea_orm::DatabaseConnection;

#[derive(Clone)]
/// Contains the shared state of the application
pub struct AppState {
    /// The database pool
    pub db: DatabaseConnection,
}
