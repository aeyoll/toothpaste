extern crate tera;

mod asset;
mod cache;
mod database;
mod routes;
mod state;

use std::{
    collections::HashMap,
    io,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, post},
    Extension, Router,
};
use cache::create_cache;
use database::create_database_pool;
use migration::{Migrator, MigratorTrait};
use routes::{
    cleanup::cleanup, create_paste::create_paste, download_paste::download_paste,
    get_paste::get_paste, index::index, new_paste::new_paste,
};
use serde_json::to_value;
use structopt::StructOpt;
use tera::{Tera, Value};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{asset::get_asset_path, state::AppState};

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    // Environment
    dotenv::dotenv().ok();

    // Logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "toothpaste=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get arguments from command line
    let args: Options = Options::from_args();

    // Database
    let db = create_database_pool().await;
    Migrator::up(&db, None).await.unwrap();

    // Cache
    let cache = create_cache();

    // Should the new pastes be private by default?
    let private = args.private;

    // State
    let shared_state = Arc::new(AppState { cache, db, private });

    // Templates
    let mut tera = Tera::new("templates/**/*.html").expect("Tera initialization failed");

    tera.register_function(
        "get_asset_path",
        |args: &HashMap<String, Value>| -> tera::Result<Value> {
            match args.get("path") {
                Some(Value::String(path)) => {
                    let res = get_asset_path(path);
                    Ok(to_value(res).unwrap())
                }
                _ => {
                    let res =
                        "invalid function usage: get_asset_path(path=\"path/to/file\")".to_string();
                    Ok(to_value(res).unwrap())
                }
            }
        },
    );

    let app = Router::with_state(shared_state)
        .route("/", get(index))
        .route("/paste/cleanup", get(cleanup))
        .route("/paste/create", get(create_paste))
        .route("/paste/new", post(new_paste))
        .route("/paste/:id", get(get_paste))
        .route("/paste/:id/download", get(download_paste))
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(handle_error),
        )
        .layer(ServiceBuilder::new().layer(Extension(tera)));

    let ip = Ipv4Addr::from_str(&*args.ip).unwrap();
    let addr = SocketAddr::from((ip, args.port));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
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
