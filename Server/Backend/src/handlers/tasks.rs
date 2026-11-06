use axum::{
    extract::{Path, State},
    Json, response::IntoResponse, http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::{AppState, models::task::Task};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub due_date: Option<chrono::DateTime<Utc>>,
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let creator_id = Uuid::nil();
    let task = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (title, description, created_by, assigned_to, due_date, status)
        VALUES ($1, $2, $3, $4, $5, 'todo')
        RETURNING *
        "#
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(creator_id)
    .bind(payload.assigned_to)
    .bind(payload.due_date)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    Ok((StatusCode::OK, Json(task)))
}

pub async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    if let Some(t) = payload.title { task.title = t; }
    if let Some(d) = payload.description { task.description = Some(d); }
    if let Some(s) = payload.status { task.status = s; }
    if let Some(dd) = payload.due_date { task.due_date = Some(dd); }

    let updated = sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks SET title = $1, description = $2, status = $3, due_date = $4, updated_at = $5
        WHERE id = $6
        RETURNING *
        "#
    )
    .bind(&task.title)
    .bind(&task.description)
    .bind(&task.status)
    .bind(task.due_date)
    .bind(Utc::now())
    .bind(task_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(updated)))
}
