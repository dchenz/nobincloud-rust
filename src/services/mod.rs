pub mod errors;
mod util;

use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

use crate::{
    model::{Account, AccountEncryptionInfo, AccountHashInfo, CreateAccountRequest, LoginRequest},
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
}
