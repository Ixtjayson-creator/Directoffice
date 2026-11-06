use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Invitation {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub department_id: Option<Uuid>,
    pub role: String,
    pub invite_code: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

