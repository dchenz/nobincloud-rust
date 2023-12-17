mod routes;

use crate::model::CreateAccountRequest;
use crate::mysql::Database;
use crate::services::Service;

use async_trait::async_trait;
use axum::{routing::post, serve, Router};
use std::sync::Arc;
use tokio::net::TcpListener;

#[async_trait]
pub trait ServiceT {
    async fn create_account(
        &self,
        request: CreateAccountRequest,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Clone)]
struct AppState {
    svc: Arc<dyn ServiceT + Sync + Send>,
}

pub async fn run(connection_string: &str) {
    let db = Database::new(connection_string).await.unwrap();

    let router = Router::new()
        .route("/api/user/register", post(routes::register_user))
        .with_state(AppState {
            svc: Arc::new(Service::new(db)),
        });

    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    serve(listener, router).await.unwrap();
}
