use crate::model::{CreateAccountRequest, Response};
use crate::server::AppState;

use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;

use tower_cookies::{Cookie, Cookies};
use tower_sessions::Session;

#[debug_handler]
pub async fn register_user(
    cookies: Cookies,
    session: Session,
    State(AppState { svc }): State<AppState>,
    Json(body): Json<CreateAccountRequest>,
) -> (StatusCode, Response<()>) {
    let svc = svc.as_ref();
    let new_id = match svc.create_account(body).await {
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::Fail(e.to_string()),
            )
        }
        Ok(v) => v,
    };
    match session.insert("user_id", new_id) {
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::Fail(e.to_string()),
            )
        }
        Ok(_) => (),
    };
    // The session cookie is HTTP-only, so a 2nd cookie
    // is used to determine if the user is signed in.
    let mut signed_in_cookie = Cookie::new("signed_in", "true");
    signed_in_cookie.set_path("/");
    cookies.add(signed_in_cookie);
    (StatusCode::OK, Response::Success(()))
}
