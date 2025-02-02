use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_account_id: String,
    pub provider_access_token: Option<String>,
    pub provider_refresh_token: Option<String>,
    pub provider_token_expires_at: Option<DateTime<Utc>>,
    pub provider_username: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Account {
    /// Creates a new account instance (not persisted)
    pub fn new(
        user_id: Uuid,
        provider: String,
        provider_account_id: String,
        provider_access_token: Option<String>,
        provider_refresh_token: Option<String>,
        provider_token_expires_at: Option<DateTime<Utc>>,
        provider_username: Option<String>,
    ) -> Self {
        Account {
            id: Uuid::new_v4(),
            user_id,
            provider,
            provider_account_id,
            provider_access_token,
            provider_refresh_token,
            provider_token_expires_at,
            provider_username,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Creates a new account in the database
    pub async fn create(
        user_id: Uuid,
        provider: &str,
        provider_account_id: &str,
        access_token: &str,
        refresh_token: &str,
        expires_at: DateTime<Utc>,
        username: &str,
        pool: &PgPool,
    ) -> Result<Account, sqlx::Error> {
        let account = Account::new(
            user_id,
            provider.to_string(),
            provider_account_id.to_string(),
            Some(access_token.to_string()),
            Some(refresh_token.to_string()),
            Some(expires_at),
            Some(username.to_string()),
        );

        sqlx::query_as::<_, Account>(
            "INSERT INTO accounts (
                id, user_id, provider, provider_account_id,
                provider_access_token, provider_refresh_token,
                provider_token_expires_at, provider_username
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *",
        )
        .bind(account.id)
        .bind(account.user_id)
        .bind(&account.provider)
        .bind(&account.provider_account_id)
        .bind(&account.provider_access_token)
        .bind(&account.provider_refresh_token)
        .bind(account.provider_token_expires_at)
        .bind(&account.provider_username)
        .fetch_one(pool)
        .await
    }

    /// Updates or creates an account in the database
    pub async fn upsert(
        user_id: Uuid,
        provider: &str,
        provider_account_id: &str,
        access_token: &str,
        refresh_token: &str,
        expires_at: DateTime<Utc>,
        username: &str,
        pool: &PgPool,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as::<_, Account>(
            "INSERT INTO accounts (
                id, user_id, provider, provider_account_id,
                provider_access_token, provider_refresh_token,
                provider_token_expires_at, provider_username
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (provider, provider_account_id)
            DO UPDATE SET
                provider_access_token = EXCLUDED.provider_access_token,
                provider_refresh_token = EXCLUDED.provider_refresh_token,
                provider_token_expires_at = EXCLUDED.provider_token_expires_at,
                provider_username = EXCLUDED.provider_username,
                updated_at = CURRENT_TIMESTAMP
            RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(provider)
        .bind(provider_account_id)
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .bind(username)
        .fetch_one(pool)
        .await
    }

    /// Finds an account by provider and provider account ID
    pub async fn find_by_provider(
        provider: &str,
        provider_account_id: &str,
        pool: &PgPool,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE provider = $1 AND provider_account_id = $2",
        )
        .bind(provider)
        .bind(provider_account_id)
        .fetch_one(pool)
        .await
    }

    /// Finds all accounts for a specific user
    pub async fn find_by_user_id(
        user_id: Uuid,
        pool: &PgPool,
    ) -> Result<Vec<Account>, sqlx::Error> {
        sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    /// Updates the tokens for an existing account
    pub async fn update_tokens(
        &self,
        access_token: &str,
        refresh_token: &str,
        expires_at: DateTime<Utc>,
        pool: &PgPool,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as::<_, Account>(
            "UPDATE accounts
            SET provider_access_token = $1,
                provider_refresh_token = $2,
                provider_token_expires_at = $3,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $4
            RETURNING *",
        )
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .bind(self.id)
        .fetch_one(pool)
        .await
    }

    /// Deletes an account
    pub async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(self.id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
