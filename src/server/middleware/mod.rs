use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

pub async fn authenticate_request(session: Session, mut request: Request, next: Next) -> Response {
    let user_id_i64 = session
        .get_value("user_id")
        .and_then(|user_id| user_id.as_i64());
    if let Some(user_id) = user_id_i64.and_then(|id| i32::try_from(id).ok()) {
        request.extensions_mut().insert(user_id);
        next.run(request).await
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
