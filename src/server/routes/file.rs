use axum::{extract::State, http::StatusCode};
use axum_typed_multipart::TypedMultipart;
use tower_sessions::Session;

use crate::{
    model::{FileUploadRequest, Response},
    server::AppState,
};

pub async fn upload_file(
    session: Session,
    State(AppState { svc }): State<AppState>,
    body: TypedMultipart<FileUploadRequest>,
) -> (StatusCode, Response<String>) {
    let user_id = match session.get_value("user_id") {
        Some(v) => i32::try_from(v.as_i64().unwrap_or_default()).unwrap_or_default(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Response::Fail("Not signed in".to_owned()),
            )
        }
    };
    let file_uuid = match svc.upload_file(user_id, body.0).await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::Fail(e.to_string()),
            )
        }
    };
    (StatusCode::OK, Response::Success(file_uuid))
}
