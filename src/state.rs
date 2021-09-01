use crate::{cache::HtmlCache, database::DatabasePool};

#[derive(Clone)]
pub struct State {
    pub cache: HtmlCache,
    pub pool: DatabasePool,
}
