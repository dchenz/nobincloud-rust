pub mod errors;
mod util;

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use sqlx::types::Uuid;
use std::sync::Arc;

use crate::{
    model::{
        Account, AccountEncryptionInfo, AccountHashInfo, CreateAccountRequest, File,
        FileUploadRequest, LoginRequest,
    },
    server::ServiceT,
};

use self::errors::{DuplicateEmail, FailedLoginAttempt};

#[async_trait]
pub trait DatabaseT {
    async fn create_account(&self, new_account: Account)
        -> Result<i32, Box<dyn std::error::Error>>;
    async fn check_email_exists(&self, email: &str) -> Result<bool, Box<dyn std::error::Error>>;
    async fn get_account_hash_info(
        &self,
        email: &str,
    ) -> Result<Option<AccountHashInfo>, Box<dyn std::error::Error>>;
    async fn get_account_encryption_key(
        &self,
        user_id: i32,
        password_hash: &[u8],
    ) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>>;
    async fn create_file(&self, new_file: File) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Clone)]
pub struct Service {
    db: Arc<dyn DatabaseT + Sync + Send>,
}

impl Service {
    pub fn new(db: impl DatabaseT + Sync + Send + 'static) -> Service {
        Service { db: Arc::new(db) }
    }
}

#[async_trait]
impl ServiceT for Service {
    async fn create_account(
        &self,
        request: CreateAccountRequest,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        if self.db.check_email_exists(&request.email).await? {
            return Err(DuplicateEmail.into());
        }
        let password_salt = util::get_rand_16();
        let password_hash = util::derive_stored_password(&request.password_hash, &password_salt);
        let new_id = self
            .db
            .create_account(Account {
                id: 0,
                nickname: request.nickname,
                email: request.email,
                created_at: Utc::now(),
                password_salt: Vec::from(password_salt),
                password_hash: Vec::from(password_hash),
                account_encryption_key: request.account_key,
            })
            .await?;
        Ok(new_id)
    }

    async fn login(
        &self,
        request: LoginRequest,
    ) -> Result<AccountEncryptionInfo, Box<dyn std::error::Error>> {
        let hash_info = self
            .db
            .get_account_hash_info(&request.email)
            .await?
            .ok_or(FailedLoginAttempt)?;
        let password_hash =
            util::derive_stored_password(&request.password_hash, &hash_info.password_salt);
        let account_encryption_key = self
            .db
            .get_account_encryption_key(hash_info.id, &password_hash)
            .await?
            .ok_or(FailedLoginAttempt)?;
        Ok(AccountEncryptionInfo {
            id: hash_info.id,
            account_encryption_key,
        })
    }

    async fn upload_file(
        &self,
        user_id: i32,
        request: FileUploadRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let file_uuid = Uuid::new_v4();
        let encryption_key = general_purpose::STANDARD.decode(request.encryption_key)?;
        let metadata = general_purpose::STANDARD.decode(request.metadata)?;
        self.db
            .create_file(File {
                id: 0,
                public_id: file_uuid.to_owned(),
                owner_id: user_id,
                encryption_key,
                parent_folder_id: None,
                metadata,
                saved_location: "test".to_owned(),
            })
            .await?;
        Ok(file_uuid.to_string())
    }
}
