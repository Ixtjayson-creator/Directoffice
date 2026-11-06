use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transcript {
    pub id: Uuid,
    pub audio_file_id: Uuid,
    pub transcript: String,
    pub created_at: DateTime<Utc>,
}

