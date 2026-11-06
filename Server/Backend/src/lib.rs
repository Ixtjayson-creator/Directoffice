pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;
pub mod middleware;
pub mod ai;

use axum::{
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::init_db_pool;
use crate::middleware::audit::audit_middleware;
use crate::handlers::ai_job::spawn_background_jobs;
use crate::handlers::{
    create_office,
    health_check,
    login,
    AppState,
    invitation::{generate_invitation, accept_invitation},
    messaging::{create_conversation, send_message, get_messages},
    voice::{upload_voice, get_voice_files},
    ai::{transcribe, summarize},
    ai_job::{create_job, get_job},
    tasks::{create_task, get_task, update_task},
    security::{register_device, revoke_device, list_devices},
};

pub async fn app(database_url: &str, jwt_secret: &[u8]) -> Result<Router, Box<dyn std::error::Error>> {
    let pool = init_db_pool(database_url).await?;

    let app_state = Arc::new(AppState {
        pool,
        jwt_secret: jwt_secret.to_vec(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/office/setup", post(create_office))
        .route("/api/auth/login", post(login))
        .route("/api/invitation/generate", post(generate_invitation))
        .route("/api/invitation/accept", post(accept_invitation))
        // Messaging
        .route("/api/messaging/conversation/create", post(create_conversation))
        .route("/api/messaging/message/send", post(send_message))
        .route("/api/messaging/message/:conversation_id", get(get_messages))
        // Voice
        .route("/api/voice/upload", post(upload_voice))
        .route("/api/voice/:conversation_id", get(get_voice_files))
        // AI
        .route("/api/ai/transcribe", post(transcribe))
        .route("/api/ai/summarize", post(summarize))
        // AI jobs
        .route("/api/ai/job", post(create_job))
        .route("/api/ai/job/:id", get(get_job))
        // Tasks
        .route("/api/task/create", post(create_task))
        .route("/api/task/:id", get(get_task))
        .route("/api/task/:id", put(update_task))
        // Device security
        .route("/api/device/register", post(register_device))
        .route("/api/device/:device_id/revoke", post(revoke_device))
        .route("/api/device/user/:user_id", get(list_devices))
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), audit_middleware))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state.clone());

    spawn_background_jobs(app_state);


    Ok(router)
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/directoffice".to_string());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "directoffice-default-super-secret-key-change-in-prod".to_string());
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    tracing::info!("Starting DirectOffice Server on {}", addr);

    let router = match app(&database_url, jwt_secret.as_bytes()).await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("Could not connect to PostgreSQL immediately: {}. Running with offline mock route.", e);
            Router::new().route("/api/health", get(health_check))
        }
    };

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

