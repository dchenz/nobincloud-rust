use crate::model::{CreateAccountRequest, Response};
use crate::server::AppState;

use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;

#[debug_handler]
pub async fn register_user(
    State(AppState { svc }): State<AppState>,
    Json(body): Json<CreateAccountRequest>,
) -> (StatusCode, Response<()>) {
    let svc = svc.as_ref();
    if let Err(e) = svc.create_account(body).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::Fail(e.to_string()),
        );
    }
    (StatusCode::OK, Response::Success(()))
}
