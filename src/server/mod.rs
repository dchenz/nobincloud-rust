mod routes;

use crate::model::CreateAccountRequest;
use crate::mysql::Database;
use crate::services::Service;

use async_trait::async_trait;
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
}

#[derive(Clone)]
struct AppState {
    svc: Arc<dyn ServiceT + Sync + Send>,
}

pub async fn run(connection_string: &str) {
    let db = Database::new(connection_string).await.unwrap();

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
        .route("/api/user/register", post(routes::register_user))
        .route("/api/user/whoami", get(routes::get_whoami))
        .route("/api/user/logout", post(routes::logout_user))
        .layer(session_service)
        .layer(CookieManagerLayer::new())
        .with_state(AppState {
            svc: Arc::new(Service::new(db)),
        });

    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    serve(listener, router).await.unwrap();
}
