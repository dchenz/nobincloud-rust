use axum::{extract::State, http::StatusCode, Extension};
use axum_typed_multipart::TypedMultipart;

use crate::{
    model::{FileUploadRequest, Response},
    server::AppState,
};

pub async fn upload_file(
    Extension(user_id): Extension<i32>,
    State(AppState { svc }): State<AppState>,
    body: TypedMultipart<FileUploadRequest>,
) -> (StatusCode, Response<String>) {
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
