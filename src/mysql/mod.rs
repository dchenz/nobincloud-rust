mod queries;

use crate::model::{Account, AccountHashInfo, File};
use crate::services::DatabaseT;

use async_trait::async_trait;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

#[derive(Clone)]
pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new(connection_string: &str) -> Result<Database, Box<dyn std::error::Error>> {
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(connection_string)
            .await?;
        Ok(Database { pool })
    }
}

#[async_trait]
impl DatabaseT for Database {
    async fn create_account(
        &self,
        new_account: Account,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let new_id: (i32,) = sqlx::query_as(queries::SQL_INSERT_ACCOUNT)
            .bind(new_account.created_at)
            .bind(new_account.nickname)
            .bind(new_account.email)
            .bind(new_account.password_salt)
            .bind(new_account.password_hash)
            .bind(new_account.account_encryption_key)
            .fetch_one(&self.pool)
            .await?;
        Ok(new_id.0)
    }

    async fn check_email_exists(&self, email: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let exists: Option<(bool,)> = sqlx::query_as(queries::SQL_CHECK_EMAIL_EXISTS)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(match exists {
            Some(v) => v.0,
            None => false,
        })
    }

    async fn get_account_hash_info(
        &self,
        email: &str,
    ) -> Result<Option<AccountHashInfo>, Box<dyn std::error::Error>> {
        let result: Option<(i32, Vec<u8>)> = sqlx::query_as(queries::SQL_GET_ACCOUNT_HASH_INFO)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(result.map(|(user_id, password_salt)| AccountHashInfo {
            id: user_id,
            password_salt,
        }))
    }

    async fn get_account_encryption_key(
        &self,
        user_id: i32,
        password_hash: &[u8],
    ) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        let result: Option<(Vec<u8>,)> = sqlx::query_as(queries::SQL_GET_ACCOUNT_ENCRYPTION_KEY)
            .bind(user_id)
            .bind(password_hash)
            .fetch_optional(&self.pool)
            .await?;
        Ok(result.map(|(key,)| key))
    }

    async fn create_file(&self, new_file: File) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(queries::SQL_INSERT_FILE)
            .bind(new_file.public_id)
            .bind(new_file.owner_id)
            .bind(new_file.encryption_key)
            .bind(new_file.parent_folder_id)
            .bind(new_file.metadata)
            .bind(new_file.saved_location)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
