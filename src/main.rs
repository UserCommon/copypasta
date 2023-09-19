mod config;
use config::Config;
use once_cell::sync::Lazy;
use serde_json::{Result, Value};
use std::string::ToString;
use axum::{routing::get, Router, ServiceExt};


async fn howdy() -> String {
    "Hello world".to_string()
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_str = include_str!("../settings.json");
    let config: Config = serde_json::from_str(config_str).expect("Failed to read settings.json");
    config
});

#[tokio::main]
async fn main() {


    let app = Router::new()
        .route("/", get(howdy));

    axum::Server::bind(&(CONFIG.to_string()).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();


    println!("AINT NO WAY!");
}
