use axum::{
    extract::{State, Path},
    Json, response::IntoResponse, http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::{AppState, models::Device};

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceRequest {
    pub user_id: Uuid,
    pub device_name: String,
    pub fingerprint: String,
}

#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_name: String,
    pub fingerprint: String,
    pub registered_at: chrono::DateTime<Utc>,
    pub revoked: bool,
}

pub async fn register_device(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterDeviceRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let now = Utc::now();
    let device = sqlx::query_as::<_, Device>(
        r#"
        INSERT INTO devices (user_id, device_name, fingerprint, registered_at, last_active, revoked)
        VALUES ($1, $2, $3, $4, $4, false)
        RETURNING *
        "#
    )
    .bind(payload.user_id)
    .bind(payload.device_name)
    .bind(payload.fingerprint)
    .bind(now)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(DeviceResponse {
        id: device.id,
        user_id: device.user_id,
        device_name: device.device_name,
        fingerprint: device.fingerprint,
        registered_at: device.registered_at,
        revoked: device.revoked,
    })))
}

pub async fn revoke_device(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query(
        "UPDATE devices SET revoked = true, last_active = $1 WHERE id = $2"
    )
    .bind(Utc::now())
    .bind(device_id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(serde_json::json!({"revoked": true}))))
}

pub async fn list_devices(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let devices = sqlx::query_as::<_, Device>(
        "SELECT * FROM devices WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(devices)))
}
