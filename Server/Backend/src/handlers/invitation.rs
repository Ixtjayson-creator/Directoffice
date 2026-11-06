use axum::{
    extract::State,
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, auth::{hash_password, generate_token}, models::{Invitation, User}};


#[derive(Debug, Deserialize)]
pub struct GenerateInvitationRequest {
    pub organization_id: Uuid,
    pub department_id: Option<Uuid>,
    pub role: Option<String>,
    pub expires_in_days: i64,
}

#[derive(Debug, Serialize)]
pub struct InvitationResponse {
    pub invite_code: String,
    pub expires_at: String,
}

pub async fn generate_invitation(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GenerateInvitationRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Only admin can call; omitted auth for brevity.
    let expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(payload.expires_in_days))
        .ok_or((StatusCode::BAD_REQUEST, "Invalid expiration".into()))?;

    let invite_code = uuid::Uuid::new_v4().to_string();

    let role = payload.role.unwrap_or_else(|| "employee".to_string());

    sqlx::query(
        r#"
        INSERT INTO invitations (organization_id, department_id, role, invite_code, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(payload.organization_id)
    .bind(payload.department_id)
    .bind(role)
    .bind(&invite_code)
    .bind(expires_at)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(InvitationResponse {
            invite_code,
            expires_at: expires_at.to_rfc3339(),
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct AcceptInvitationRequest {
    pub invite_code: String,
    pub username: String,
    pub full_name: String,
    pub password: String,
    pub device_name: String,
    pub fingerprint: String,
}

pub async fn accept_invitation(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AcceptInvitationRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Validate invitation
    let invitation = sqlx::query_as::<_, Invitation>(
        "SELECT * FROM invitations WHERE invite_code = $1 AND used = FALSE AND expires_at > NOW()",
    )
    .bind(&payload.invite_code)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::BAD_REQUEST, "Invalid or expired invitation".into()))?;

    // Create user
    let password_hash = hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    let role = invitation.role.clone();
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (organization_id, department_id, username, full_name, password_hash, role, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'active')
        RETURNING *
        "#,
    )
    .bind(invitation.organization_id)
    .bind(invitation.department_id)
    .bind(&payload.username)
    .bind(&payload.full_name)
    .bind(password_hash)
    .bind(&role)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Register device
    sqlx::query(
        r#"
        INSERT INTO devices (user_id, device_name, fingerprint)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(&payload.device_name)
    .bind(&payload.fingerprint)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Mark invitation used
    sqlx::query(
        "UPDATE invitations SET used = TRUE WHERE id = $1",
    )
    .bind(invitation.id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Generate token
    let token = generate_token(user.id, user.organization_id, &user.role, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "token": token,
            "user": user,
        })),
    ))
}
