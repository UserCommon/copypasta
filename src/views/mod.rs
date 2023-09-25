#![allow(dead_code, unused_imports)]
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post, Router},
    Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{model::*, schema::*, AppState};

#[derive(Debug, Deserialize)]
pub struct Params {
    name: Option<String>,
}

pub fn pasta_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/check", get(check))
        .route("/", get(pasta_list))
        .with_state(state.clone())
}

pub async fn check() -> impl IntoResponse {
    let resp = serde_json::json!({
        "status": "success",
        "message": "I'm alive"
    });
    Json(resp)
}

pub async fn pasta_list(
    filter: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(filter) = filter.unwrap_or_default();

    let limit = filter.limit.unwrap_or(10);
    let offset = (filter.page.unwrap_or(1) - 1) * limit;

    let pastas = sqlx::query_as!(
        PastaModel,
        r#"SELECT * FROM pasta ORDER BY id LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let err_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error {}", e)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(err_response))
    })?;

    let json_response = serde_json::json!({
        "status": "success",
        "results": pastas.len(),
        "pastas": pastas
    });
    Ok(Json(json_response))
}
