mod database;
mod routes;
mod state;

use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use axum::{
    routing::{get, post},
    Router,
};
use database::create_database_pool;
use migration::{Migrator, MigratorTrait};
use routes::{cleanup::cleanup, get_paste::get_paste, new_paste::new_paste};
use structopt::StructOpt;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
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

    // State
    let shared_state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/api/paste/cleanup", get(cleanup))
        .route("/api/paste/new", post(new_paste))
        .route("/api/paste/:id", get(get_paste))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(shared_state);

    let ip = Ipv4Addr::from_str(&args.ip).unwrap();
    let addr = SocketAddr::from((ip, args.port));
    tracing::debug!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "toothpaste", about)]
struct Options {
    #[structopt(long, default_value = "127.0.0.1")]
    ip: String,

    #[structopt(long, default_value = "8080")]
    port: u16,
}
