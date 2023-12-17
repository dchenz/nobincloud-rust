use super::serdebase64;
use axum::{response::IntoResponse, Json};
use either::Either;
use serde;

#[derive(Debug, serde::Deserialize)]
pub struct CreateAccountRequest {
    pub email: String,
    pub nickname: String,
    #[serde(rename = "passwordHash", with = "serdebase64")]
    pub password_hash: Vec<u8>,
    #[serde(rename = "accountKey", with = "serdebase64")]
    pub account_key: Vec<u8>,
}

#[derive(serde::Serialize)]
struct ResponseBody<T: serde::Serialize> {
    success: bool,
    #[serde(with = "either::serde_untagged")]
    data: Either<T, String>,
}

pub enum Response<T> {
    Success(T),
    Fail(String),
}

impl<T: serde::Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Response::Success(data) => ResponseBody {
                success: true,
                data: Either::Left(data),
            },
            Response::Fail(reason) => ResponseBody {
                success: false,
                data: Either::Right(reason),
            },
        };
        Json(body).into_response()
    }
}
