use crate::{cache::HtmlCache, database::DatabasePool};

#[derive(Clone)]
/// Contains the shared state of the application
pub struct State {
    /// The cache
    pub cache: HtmlCache,
    /// The database pool
    pub pool: DatabasePool,
    /// Are the paste created as private by default?
    pub private: bool,
}
