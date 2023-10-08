mod config;
mod model;
mod schema;
mod tests;
mod views;

use anyhow::Context;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{get, get_service},
    Router, ServiceExt,
};
use config::{Config, Urls};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde_json::{Result, Value};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./static/")))
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_str = include_str!("../settings.json");
    let config: Config = serde_json::from_str(config_str).expect("Failed to read settings.json");
    config
});

pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = CONFIG.get_db_url();
    println!("=> Trying to connect to {}", db_url);
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Failed to connect database");

    println!("=> Making migrations...");
    sqlx::migrate!().run(&db).await.expect("failed to migrate");

    let cors = CorsLayer::new()
        .allow_origin(CONFIG.get_url().parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    println!("=> RUNNING ON {}!", CONFIG.get_url());
    println!("=> Config: {conf:#?}", conf = CONFIG);
    println!("=> Config_URL: {}", CONFIG.get_url());

    let state = Arc::new(AppState { db });

    let app = Router::new()
        .with_state(state.clone())
        .layer(cors)
        .nest("/pastas", views::pasta_routes(state.clone()))
        .fallback_service(route_static());

    axum::Server::bind(&(CONFIG.get_url()).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("AINT NO WAY!");
}
