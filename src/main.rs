mod config;
mod db;
mod error;
mod models;
mod routes;
mod shortcode;

use axum::routing::{delete, get, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use config::Config;
use db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let pool = db::init_pool(&config.database_path);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("Starting server on {addr}");
    tracing::info!("Base URL: {}", config.base_url);

    let state = AppState { pool, config };

    let app = Router::new()
        .route("/api/shorten", post(routes::create::create_url))
        .route("/api/stats/{code}", get(routes::stats::get_stats))
        .route("/api/urls/{code}", delete(routes::delete::delete_url))
        .route("/{code}", get(routes::redirect::redirect))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
