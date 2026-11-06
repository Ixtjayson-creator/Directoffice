use axum::{
    extract::{State, Path},
    Json, response::IntoResponse, http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, models::voice::AudioFile};

#[derive(Debug, Deserialize)]
pub struct UploadVoiceRequest {
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub file_path: String,
    pub duration_seconds: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UploadVoiceResponse {
    pub id: Uuid,
    pub file_path: String,
}

pub async fn upload_voice(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UploadVoiceRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let record = sqlx::query_as::<_, AudioFile>(
        r#"
        INSERT INTO audio_files (conversation_id, sender_id, file_path, duration_seconds)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(payload.conversation_id)
    .bind(payload.sender_id)
    .bind(&payload.file_path)
    .bind(payload.duration_seconds)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(UploadVoiceResponse { id: record.id, file_path: record.file_path })))
}

pub async fn get_voice_files(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let files = sqlx::query_as::<_, AudioFile>(
        "SELECT * FROM audio_files WHERE conversation_id = $1 ORDER BY created_at ASC"
    )
    .bind(conversation_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(files)))
}

