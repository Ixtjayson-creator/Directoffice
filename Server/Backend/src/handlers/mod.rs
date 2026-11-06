use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::{hash_password, generate_token, verify_password};
use crate::models::{Organization, User};

pub mod invitation;
pub mod messaging;
pub mod voice;
pub mod ai;
pub mod ai_job;
pub mod tasks;
pub mod security;

pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOfficeRequest {
    pub office_name: String,
    pub office_code: String,
    pub timezone: Option<String>,
    pub admin_username: String,
    pub admin_full_name: String,
    pub admin_password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
    pub organization: Organization,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub office_code: String,
    pub username: String,
    pub password: String,
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok", "service": "directoffice-server" })))
}

pub async fn create_office(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateOfficeRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let timezone = payload.timezone.unwrap_or_else(|| "UTC".to_string());

    let org = sqlx::query_as::<_, Organization>(
        r#"
        INSERT INTO organizations (name, office_code, timezone)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&payload.office_name)
    .bind(&payload.office_code)
    .bind(&timezone)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to create organization: {}", e)))?;

    let password_hash = hash_password(&payload.admin_password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (organization_id, username, full_name, password_hash, role, status)
        VALUES ($1, $2, $3, $4, 'admin', 'active')
        RETURNING *
        "#,
    )
    .bind(org.id)
    .bind(&payload.admin_username)
    .bind(&payload.admin_full_name)
    .bind(password_hash)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to create admin user: {}", e)))?;

    tx.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let token = generate_token(user.id, org.id, &user.role, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user,
            organization: org,
        }),
    ))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let org = sqlx::query_as::<_, Organization>(
        "SELECT * FROM organizations WHERE office_code = $1",
    )
    .bind(&payload.office_code)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "Invalid office code or credentials".to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE organization_id = $1 AND username = $2",
    )
    .bind(org.id)
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "Invalid office code or credentials".to_string()))?;

    if !verify_password(&payload.password, &user.password_hash) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid office code or credentials".to_string()));
    }

    let token = generate_token(user.id, org.id, &user.role, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((
        StatusCode::OK,
        Json(AuthResponse {
            token,
            user,
            organization: org,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub username: String,
    pub full_name: String,
    pub password: String,
    pub department_id: Option<Uuid>,
    pub role: Option<String>,
}

pub async fn create_employee(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<CreateEmployeeRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _password_hash = hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let _role = payload.role.unwrap_or_else(|| "employee".to_string());

    // In a full implementation, auth middleware extracts org_id from JWT claims
    // Here we provide the endpoint handler pattern
    Ok((StatusCode::OK, Json(serde_json::json!({ "message": "employee endpoint ready" }))))
}

