mod middleware;
mod routes;

use crate::model::{AccountEncryptionInfo, CreateAccountRequest, FileUploadRequest, LoginRequest};
use crate::mysql::Database;
use crate::services::Service;

use async_trait::async_trait;
use axum::middleware::from_fn;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    serve, BoxError, Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};

#[async_trait]
pub trait ServiceT {
    async fn create_account(
        &self,
        request: CreateAccountRequest,
    ) -> Result<i32, Box<dyn std::error::Error>>;

    async fn login(
        &self,
        request: LoginRequest,
    ) -> Result<AccountEncryptionInfo, Box<dyn std::error::Error>>;

    async fn upload_file(
        &self,
        user_id: i32,
        request: FileUploadRequest,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
struct AppState {
    svc: Arc<dyn ServiceT + Sync + Send>,
}

pub struct ServerConfig {
    pub hostname: String,
    pub port: u16,
    pub db_connection_string: String,
}

pub async fn run(config: ServerConfig) {
    let db = Database::new(&config.db_connection_string).await.unwrap();

    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(
            SessionManagerLayer::new(MemoryStore::default())
                .with_http_only(true)
                .with_secure(false)
                .with_expiry(Expiry::OnInactivity(Duration::hours(24))),
        );

    let router = Router::new()
        .route("/api/file", post(routes::upload_file))
        .layer(from_fn(middleware::authenticate_request))
        .route("/api/user/register", post(routes::register_user))
        .route("/api/user/whoami", get(routes::get_whoami))
        .route("/api/user/login", post(routes::login_user))
        .route("/api/user/logout", post(routes::logout_user))
        .layer(session_service)
        .layer(CookieManagerLayer::new())
        .with_state(AppState {
            svc: Arc::new(Service::new(db)),
        });

    let addr = format!("{}:{}", config.hostname, config.port);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, router).await.unwrap();
}
