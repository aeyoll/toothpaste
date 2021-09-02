#[macro_use]
extern crate rbatis;

mod cache;
mod database;
mod paste;
mod routes;
mod state;
mod template;

use cache::create_cache;
use database::create_database_pool;
use routes::get_paste::get_paste;
use routes::index::index;
use routes::new::new;
use state::State;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Environment
    dotenv::dotenv().ok();

    // Database
    let pool = create_database_pool().await;

    // Cache
    let cache = create_cache();

    // State
    let state = State { cache, pool };

    let mut app = tide::with_state(state.clone());

    app.at("/").get(index);
    app.at("/paste/new").post(new);
    app.at("/paste/:id").get(get_paste);
    app.at("/static").serve_dir("static/")?;

    let addr = "127.0.0.1:8080";
    println!("http server listen on http://{}", addr);

    app.listen(addr).await?;
    Ok(())
}
