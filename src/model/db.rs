use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(Debug)]
pub struct Account {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub email: String,
    pub nickname: String,
    pub password_salt: Vec<u8>,
    pub password_hash: Vec<u8>,
    pub account_encryption_key: Vec<u8>,
}

#[derive(Debug)]
pub struct AccountHashInfo {
    pub id: i32,
    pub password_salt: Vec<u8>,
}

#[derive(Debug)]
pub struct AccountEncryptionInfo {
    pub id: i32,
    pub account_encryption_key: Vec<u8>,
}

#[derive(Debug)]
pub struct File {
    pub id: i32,
    pub public_id: Uuid,
    pub owner_id: i32,
    pub encryption_key: Vec<u8>,
    pub parent_folder_id: Option<i32>,
    pub metadata: Vec<u8>,
    pub saved_location: String,
}
