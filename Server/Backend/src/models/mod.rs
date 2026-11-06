use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod invitation;
pub mod messaging;
pub mod voice;
pub mod task;
pub mod job;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuditEvent {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub office_code: String,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Department {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub department_id: Option<Uuid>,
    pub username: String,
    pub full_name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_name: String,
    pub fingerprint: String,
    pub registered_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub revoked: bool,
}

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

