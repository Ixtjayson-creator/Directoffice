use axum::{
    extract::{Path, State},
    Json, response::IntoResponse, http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, models::messaging::{Conversation, Message}};

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    pub is_group: bool,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    pub id: Uuid,
    pub is_group: bool,
    pub name: Option<String>,
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let convo = sqlx::query_as::<_, Conversation>(
        r#"
        INSERT INTO conversations (is_group, name)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(payload.is_group)
    .bind(payload.name)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ConversationResponse { id: convo.id, is_group: convo.is_group, name: convo.name })))
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: Uuid,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub timestamp: String,
    pub status: String,
}

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let sender_id = Uuid::nil();
    let msg = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (conversation_id, sender_id, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(payload.conversation_id)
    .bind(sender_id)
    .bind(payload.content)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(MessageResponse {
        id: msg.id,
        conversation_id: msg.conversation_id,
        sender_id: msg.sender_id,
        content: msg.content,
        timestamp: msg.timestamp.to_rfc3339(),
        status: msg.status,
    })))
}

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let msgs = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE conversation_id = $1 ORDER BY timestamp ASC"
    )
    .bind(conversation_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let resp: Vec<MessageResponse> = msgs.into_iter().map(|m| MessageResponse {
        id: m.id,
        conversation_id: m.conversation_id,
        sender_id: m.sender_id,
        content: m.content,
        timestamp: m.timestamp.to_rfc3339(),
        status: m.status,
    }).collect();

    Ok((StatusCode::OK, Json(resp)))
}
