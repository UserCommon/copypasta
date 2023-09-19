mod config;
mod tests;
mod views;

use config::Config;
use once_cell::sync::Lazy;
use serde_json::{Result, Value};
use std::string::ToString;
use axum::{routing::{get, get_service}, Router, ServiceExt};
use tower_http::services::ServeDir;

fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}


pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_str = include_str!("../settings.json");
    let config: Config = serde_json::from_str(config_str).expect("Failed to read settings.json");
    config
});

#[tokio::main]
async fn main() {

    let app = Router::new()
                        .merge(views::hello_routes())
                        .fallback_service(route_static());

    println!("=> RUNNING ON {}", CONFIG.to_string());
    axum::Server::bind(&(CONFIG.to_string()).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();


    println!("AINT NO WAY!");
}
