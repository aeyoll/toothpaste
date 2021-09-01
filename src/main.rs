#[macro_use]
extern crate rbatis;

use rbatis::crud::CRUD;
use tide::Request;

mod cache;
mod database;
mod paste;
mod routes;
mod state;
mod template;

use cache::create_cache;
use database::create_database_pool;
use paste::Paste;
use routes::get_paste::get_paste;
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
    // app.at("/new").post(new);
    app.at("/paste/:id").get(get_paste);

    let addr = "127.0.0.1:8080";
    println!("http server listen on http://{}", addr);

    app.listen(addr).await?;
    Ok(())
}

pub async fn index(req: Request<State>) -> tide::Result<String> {
    let state = req.state();
    let pool = state.pool.lock().await;
    let v = pool.fetch_list::<Paste>().await;
    Ok(serde_json::json!(v).to_string())
}

// pub async fn new(req: Request<State>) -> tide::Result<String> {
//     Ok("".to_string())
// }
