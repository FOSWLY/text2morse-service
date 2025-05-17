mod api;
mod data;
mod routes;
mod templates;
mod utils;

use axum::{
    Router,
    routing::{delete, get, post},
};
use dotenv::dotenv;
use moka::future::Cache;
use tower_http::services::ServeDir;

use crate::data::config::CONFIG;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cache: Cache<String, String> = Cache::builder()
        .time_to_live(CONFIG.cache_ttl)
        .max_capacity(16384)
        .build();

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(routes::index::get_index))
        .route("/robots.txt", get(routes::index::get_robots_txt))
        .route("/legal/{*legal_name}", get(routes::legal::get_legal))
        .route("/my/profile", get(routes::profile::get_my_profile))
        .route("/auth/callback", get(routes::auth::get_auth_callback))
        .route("/auth/logout", get(routes::auth::get_auth_logout))
        .route("/v1/auth/handle", get(routes::auth::get_auth_handle))
        .route("/v1/auth/login", post(routes::auth::post_auth_login))
        .route("/v1/auth/logout", delete(routes::auth::delete_auth_logout))
        .route("/v1/health", get(routes::health::get_health))
        .with_state(cache);

    let listener = tokio::net::TcpListener::bind(format!("{0}:{1}", CONFIG.hostname, CONFIG.port))
        .await
        .unwrap();
    println!("🦀 Axum is running at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
