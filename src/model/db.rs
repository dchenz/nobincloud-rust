use chrono::{DateTime, Utc};

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
