#[macro_use]
extern crate rbatis;

mod asset;
mod cache;
mod database;
mod paste;
mod routes;
mod state;
mod templates;

use cache::create_cache;
use database::create_database_pool;
use routes::cleanup::cleanup;
use routes::create::create;
use routes::download_paste::download_paste;
use routes::get_paste::get_paste;
use routes::index::index;
use routes::new::new;
use state::State;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Environment
    dotenv::dotenv().ok();

    // Get arguments from command line
    let args: Options = Options::from_args();

    // Database
    let pool = create_database_pool().await;

    // Cache
    let cache = create_cache();

    // Should the new pastes be private by default?
    let private = args.private;

    // State
    let state = State {
        cache,
        pool,
        private,
    };

    let mut app = tide::with_state(state.clone());

    app.at("/").get(index);
    app.at("/paste/cleanup").get(cleanup);
    app.at("/paste/create").get(create);
    app.at("/paste/new").post(new);
    app.at("/paste/:id").get(get_paste);
    app.at("/paste/:id/download").get(download_paste);
    app.at("/static").serve_dir("static/")?;

    let addr = format!("{}:{}", &args.ip, args.port);
    println!("http server listen on http://{}", addr);

    app.listen(addr).await?;
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "toothpaste", about)]
struct Options {
    #[structopt(long, default_value = "127.0.0.1")]
    ip: String,

    #[structopt(long, default_value = "8080")]
    port: u16,

    #[structopt(long)]
    private: bool,
}
