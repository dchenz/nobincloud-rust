mod queries;

use crate::model::Account;
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
        Ok(Database { pool: pool })
    }
}

#[async_trait]
impl DatabaseT for Database {
    async fn create_account(
        &self,
        new_account: Account,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let new_id: (i32,) = sqlx::query_as(queries::SQL_INSERT_ACCOUNT)
            .bind(&new_account.created_at)
            .bind(&new_account.nickname)
            .bind(&new_account.email)
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
}
