#![allow(dead_code, unused_imports)]
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, Router},
    Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::{model::*, schema::*, AppState};

#[derive(Debug, Deserialize)]
pub struct Params {
    name: Option<String>,
}

pub fn pasta_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/check", get(check))
        .route("/", get(pasta_list).post(pasta_create))
        .route("/:id", delete(delete_pasta).get(read_pasta))
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

pub async fn read_pasta(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pasta = sqlx::query_as!(
        PastaModel,
        r#"
            SELECT * FROM pasta WHERE id = $1
        "#,
        id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "failed",
                "message": format!("{:?}", e)
            })),
        )
    })?;

    let resp = json!({
        "status": "succsess",
        "data": pasta
    });
    Ok(Json(resp))
}

pub async fn pasta_create(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePastaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = uuid::Uuid::new_v4();
    let query_result = sqlx::query(
        r#"
        INSERT INTO pasta (id, lang, text) VALUES ($1, $2, $3) 
        "#,
    )
    .bind(user_id)
    .bind(body.lang.to_string())
    .bind(body.text.to_string())
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Pasta duplicate!"
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", err)
            })),
        ));
    }

    let pasta = sqlx::query_as!(
        PastaModel,
        r#"
            SELECT * FROM pasta WHERE id = $1
        "#,
        user_id,
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e),
            })),
        )
    })?;

    let pasta_resp = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "pasta": pasta
        })
    });
    Ok(Json(pasta_resp))
}

pub async fn delete_pasta(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query!(r#"DELETE FROM pasta WHERE id=$1"#, id)
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;
    if query.rows_affected() == 0 {
        let err_response = serde_json::json!({
            "status": "fail",
            "message": format!("Pasta with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(err_response)));
    }
    let resp = serde_json::json!({
        "status": "success",
    });
    Ok(Json(resp))
}
