use axum::{response::{IntoResponse, Json, Html}, extract::Query, routing::{Router, get, post}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    name: Option<String>
}

pub fn hello_routes() -> Router {
    Router::new()
        .route("/hello_world", get(howdy))
}


pub async fn howdy(Query(params): Query<Params>) -> impl IntoResponse {
    let param = params.name.as_deref().unwrap_or("world!");
    Html(format!("Hello <strong>{param}</strong>"))
}