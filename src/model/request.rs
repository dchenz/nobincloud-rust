use super::serdebase64;
use axum::{body::Bytes, response::IntoResponse, Json};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use either::Either;

#[derive(Debug, serde::Deserialize)]
pub struct CreateAccountRequest {
    pub email: String,
    pub nickname: String,
    #[serde(rename = "passwordHash", with = "serdebase64")]
    pub password_hash: Vec<u8>,
    #[serde(rename = "accountKey", with = "serdebase64")]
    pub account_key: Vec<u8>,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    #[serde(rename = "passwordHash", with = "serdebase64")]
    pub password_hash: Vec<u8>,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    #[serde(rename = "accountKey", with = "serdebase64")]
    pub account_encryption_key: Vec<u8>,
}

#[derive(Debug, TryFromMultipart)]
pub struct FileUploadRequest {
    pub file: FieldData<Bytes>,
    #[form_data(field_name = "encryptionKey")]
    pub encryption_key: String,
    #[form_data(field_name = "parentFolder")]
    pub parent_folder: Option<String>,
    pub metadata: String,
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
