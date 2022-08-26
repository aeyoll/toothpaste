use sea_orm::DatabaseConnection;

use crate::cache::HtmlCache;

#[derive(Clone)]
/// Contains the shared state of the application
pub struct AppState {
    /// The cache
    pub cache: HtmlCache,

    /// The database pool
    pub db: DatabaseConnection,

    /// Are the paste created as private by default?
    pub private: bool,
}
