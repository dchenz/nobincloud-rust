use crate::model::{CreateAccountRequest, LoginRequest, LoginResponse, Response};
use crate::server::AppState;
use crate::services::errors::{DuplicateEmail, FailedLoginAttempt};

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
    let email = body.email.to_owned();
    let new_id = match svc.create_account(body).await {
        Err(e) => {
            return (
                if e.is::<DuplicateEmail>() {
                    StatusCode::OK
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                },
                Response::Fail(e.to_string()),
            )
        }
        Ok(v) => v,
    };
    if let Err(e) = session.insert("user_id", new_id) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::Fail(e.to_string()),
        );
    }
    if let Err(e) = session.insert("user_email", email) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::Fail(e.to_string()),
        );
    }
    // The session cookie is HTTP-only, so a 2nd cookie
    // is used to determine if the user is signed in.
    let mut signed_in_cookie = Cookie::new("signed_in", "true");
    signed_in_cookie.set_path("/");
    cookies.add(signed_in_cookie);
    (StatusCode::OK, Response::Success(()))
}

pub async fn get_whoami(session: Session) -> (StatusCode, Response<String>) {
    let email = match session.get_value("user_email").unwrap_or_default() {
        sqlx::types::JsonValue::String(s) => s,
        _ => "".to_owned(),
    };
    (StatusCode::OK, Response::Success(email))
}

pub async fn login_user(
    cookies: Cookies,
    session: Session,
    State(AppState { svc }): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> (StatusCode, Response<LoginResponse>) {
    let email = body.email.to_owned();
    let result = match svc.login(body).await {
        Ok(v) => v,
        Err(e) => {
            return if e.is::<FailedLoginAttempt>() {
                (StatusCode::UNAUTHORIZED, Response::Fail(e.to_string()))
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Response::Fail(e.to_string()),
                )
            }
        }
    };
    if let Err(e) = session.insert("user_id", result.id) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::Fail(e.to_string()),
        );
    }
    if let Err(e) = session.insert("user_email", email) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Response::Fail(e.to_string()),
        );
    }
    // The session cookie is HTTP-only, so a 2nd cookie
    // is used to determine if the user is signed in.
    let mut signed_in_cookie = Cookie::new("signed_in", "true");
    signed_in_cookie.set_path("/");
    cookies.add(signed_in_cookie);
    let response_body = LoginResponse {
        account_encryption_key: result.account_encryption_key,
    };
    (StatusCode::OK, Response::Success(response_body))
}

pub async fn logout_user(session: Session) -> (StatusCode, Response<()>) {
    session.flush();
    (StatusCode::OK, Response::Success(()))
}
