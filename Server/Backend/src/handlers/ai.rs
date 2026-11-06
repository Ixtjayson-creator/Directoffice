use axum::{extract::{State, Json}, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{AppState, ai};

#[derive(Debug, Deserialize)]
pub struct TranscribeRequest {
    pub audio_path: String,
}

#[derive(Debug, Serialize)]
pub struct TranscribeResponse {
    pub transcript: String,
}

pub async fn transcribe(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<TranscribeRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ai::transcribe(std::path::PathBuf::from(&payload.audio_path)).await {
        Ok(text) => Ok((StatusCode::OK, Json(TranscribeResponse { transcript: text }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct SummarizeRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct SummarizeResponse {
    pub summary: String,
}

pub async fn summarize(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<SummarizeRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ai::summarize(&payload.text).await {
        Ok(summary) => Ok((StatusCode::OK, Json(SummarizeResponse { summary }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
