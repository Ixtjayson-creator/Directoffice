use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use sqlx::PgPool;
use tokio::time::{sleep, Duration};

use crate::{AppState, models::job::AiJob, ai};

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub conversation_id: Uuid,
    pub job_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub id: Uuid,
    pub status: String,
    pub result: Option<serde_json::Value>,
}

pub async fn create_job(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateJobRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let creator = Uuid::nil();
    let job = sqlx::query_as::<_, AiJob>(
        r#"
        INSERT INTO ai_jobs (conversation_id, user_id, job_type, payload, status)
        VALUES ($1, $2, $3, $4, 'queued')
        RETURNING *
        "#,
    )
    .bind(req.conversation_id)
    .bind(creator)
    .bind(&req.job_type)
    .bind(&req.payload)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(JobResponse { id: job.id, status: job.status, result: job.result })))
}

pub async fn get_job(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let job = sqlx::query_as::<_, AiJob>("SELECT * FROM ai_jobs WHERE id = $1")
        .bind(job_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Job not found".to_string()))?;

    Ok((StatusCode::OK, Json(JobResponse { id: job.id, status: job.status, result: job.result })))
}

async fn fetch_next_queued(pool: &PgPool) -> Result<Option<AiJob>, sqlx::Error> {
    sqlx::query_as::<_, AiJob>(
        "SELECT * FROM ai_jobs WHERE status = 'queued' ORDER BY created_at ASC LIMIT 1",
    )
    .fetch_optional(pool)
    .await
}

async fn process_job(job: AiJob, pool: &PgPool) -> Result<(), sqlx::Error> {
    let result = match job.job_type.as_str() {
        "transcribe" => {
            let path: String = job.payload["file_path"].as_str().unwrap_or_default().to_string();
            match ai::transcribe(std::path::PathBuf::from(path)).await {
                Ok(text) => Some(serde_json::json!({ "transcript": text })),
                Err(_) => None,
            }
        }
        "summarize" => {
            let text: String = job.payload["text"].as_str().unwrap_or_default().to_string();
            match ai::summarize(&text).await {
                Ok(summary) => Some(serde_json::json!({ "summary": summary })),
                Err(_) => None,
            }
        }
        _ => None,
    };

    let new_status = if result.is_some() { "completed" } else { "failed" };
    sqlx::query("UPDATE ai_jobs SET status = $1, result = $2, updated_at = $3 WHERE id = $4")
        .bind(new_status)
        .bind(result)
        .bind(Utc::now())
        .bind(job.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub fn spawn_background_jobs(state: Arc<AppState>) {
    let pool = state.pool.clone();
    tokio::spawn(async move {
        loop {
            match fetch_next_queued(&pool).await {
                Ok(Some(job)) => {
                    let _ = process_job(job, &pool).await;
                }
                Ok(None) => {
                    sleep(Duration::from_secs(2)).await;
                }
                Err(_) => {
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}
