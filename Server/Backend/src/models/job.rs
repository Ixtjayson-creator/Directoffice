use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AiJob {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub job_type: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub result: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

