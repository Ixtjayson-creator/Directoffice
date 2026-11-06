use axum::{
    extract::{State, Request},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::AppState;

pub async fn audit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();
    let action = format!("{} {}", method, path);

    let pool = state.pool.clone();
    tokio::spawn(async move {
        let _ = sqlx::query(
            "INSERT INTO audit_events (organization_id, user_id, action, created_at) VALUES ($1, $2, $3, $4)"
        )
        .bind(Uuid::nil())
        .bind(Option::<Uuid>::None)
        .bind(action)
        .bind(Utc::now())
        .execute(&pool)
        .await;
    });

    next.run(req).await
}
