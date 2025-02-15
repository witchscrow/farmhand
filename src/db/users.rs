use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    password_hash: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub settings: Option<UserSettings>,
    pub accounts: Vec<Account>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stream_status_enabled: Option<DateTime<Utc>>,
    pub chat_messages_enabled: Option<DateTime<Utc>>,
    pub channel_points_enabled: Option<DateTime<Utc>>,
    pub follows_subs_enabled: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
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

#[derive(sqlx::FromRow)]
struct UserWithSettingsAndAccount {
    // User fields
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    // Settings fields
    pub settings_id: Option<Uuid>,
    pub stream_status_enabled: Option<DateTime<Utc>>,
    pub chat_messages_enabled: Option<DateTime<Utc>>,
    pub channel_points_enabled: Option<DateTime<Utc>>,
    pub follows_subs_enabled: Option<DateTime<Utc>>,
    pub settings_created_at: Option<DateTime<Utc>>,
    pub settings_updated_at: Option<DateTime<Utc>>,
    // Account fields
    pub account_id: Option<Uuid>,
    pub provider: Option<String>,
    pub provider_account_id: Option<String>,
    pub provider_access_token: Option<String>,
    pub provider_refresh_token: Option<String>,
    pub provider_token_expires_at: Option<DateTime<Utc>>,
    pub provider_username: Option<String>,
    pub account_created_at: Option<DateTime<Utc>>,
    pub account_updated_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Creator,
    Viewer,
}

pub enum UserError {
    FailedToHashPassword,
    BadPassword,
}

impl User {
    /// Creates a new user from the given parameters
    // NOTE: This does not hash the password by default
    pub fn new(email: String, username: String, password_hash: String) -> Self {
        let id = Uuid::new_v4();
        User {
            id,
            email,
            username,
            password_hash,
            role: UserRole::Viewer,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            settings: None,
            accounts: Vec::new(),
        }
    }
    /// Gets a user from the databased based on Username
    pub async fn by_username(username: String, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let rows = sqlx::query_as::<_, UserWithSettingsAndAccount>(
            r#"
                SELECT
                    u.*,
                    s.id as settings_id,
                    s.stream_status_enabled,
                    s.chat_messages_enabled,
                    s.channel_points_enabled,
                    s.follows_subs_enabled,
                    s.created_at as settings_created_at,
                    s.updated_at as settings_updated_at,
                    a.id as account_id,
                    a.provider,
                    a.provider_account_id,
                    a.provider_access_token,
                    a.provider_refresh_token,
                    a.provider_token_expires_at,
                    a.provider_username,
                    a.created_at as account_created_at,
                    a.updated_at as account_updated_at
                FROM users u
                LEFT JOIN user_settings s ON s.user_id = u.id
                LEFT JOIN accounts a ON a.user_id = u.id
                WHERE u.username = $1
                "#,
        )
        .bind(username)
        .fetch_all(pool)
        .await?;

        if rows.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        let first_row = &rows[0];

        // Convert the rows into accounts
        // Convert the rows into accounts
        let accounts = rows
            .iter()
            .filter_map(|row| {
                // Only create account if we have all required fields
                row.account_id.and_then(|account_id| {
                    // Check for required fields
                    let provider = row.provider.clone()?;
                    let provider_account_id = row.provider_account_id.clone()?;
                    let created_at = row.account_created_at?;
                    let updated_at = row.account_updated_at?;

                    Some(Account {
                        id: account_id,
                        user_id: first_row.id,
                        provider,
                        provider_account_id,
                        provider_access_token: row.provider_access_token.clone(),
                        provider_refresh_token: row.provider_refresh_token.clone(),
                        provider_token_expires_at: row.provider_token_expires_at,
                        provider_username: row.provider_username.clone(),
                        created_at,
                        updated_at,
                    })
                })
            })
            .collect();

        let settings = first_row.settings_id.and_then(|settings_id| {
            // Only create settings if we have the required timestamps
            match (first_row.settings_created_at, first_row.settings_updated_at) {
                (Some(created_at), Some(updated_at)) => Some(UserSettings {
                    id: settings_id,
                    user_id: first_row.id,
                    stream_status_enabled: first_row.stream_status_enabled,
                    chat_messages_enabled: first_row.chat_messages_enabled,
                    channel_points_enabled: first_row.channel_points_enabled,
                    follows_subs_enabled: first_row.follows_subs_enabled,
                    created_at,
                    updated_at,
                }),
                _ => None,
            }
        });

        Ok(User {
            id: first_row.id,
            email: first_row.email.clone(),
            username: first_row.username.clone(),
            password_hash: first_row.password_hash.clone(),
            role: first_row.role.clone(),
            created_at: first_row.created_at,
            updated_at: first_row.updated_at,
            settings,
            accounts,
        })
    }
    /// Gets a user from the database based on ID
    pub async fn by_id(id: Uuid, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let rows = sqlx::query_as::<_, UserWithSettingsAndAccount>(
            r#"
                SELECT
                    u.*,
                    s.id as settings_id,
                    s.stream_status_enabled,
                    s.chat_messages_enabled,
                    s.channel_points_enabled,
                    s.follows_subs_enabled,
                    s.created_at as settings_created_at,
                    s.updated_at as settings_updated_at,
                    a.id as account_id,
                    a.provider,
                    a.provider_account_id,
                    a.provider_access_token,
                    a.provider_refresh_token,
                    a.provider_token_expires_at,
                    a.provider_username,
                    a.created_at as account_created_at,
                    a.updated_at as account_updated_at
                FROM users u
                LEFT JOIN user_settings s ON s.user_id = u.id
                LEFT JOIN accounts a ON a.user_id = u.id
                WHERE u.id = $1
                "#,
        )
        .bind(id)
        .fetch_all(pool) // Fetch all to get multiple accounts
        .await?;

        if rows.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        let first_row = &rows[0];

        // Convert the rows into accounts
        let accounts = rows
            .iter()
            .filter_map(|row| {
                match (
                    row.account_id,
                    row.provider.clone(),
                    row.provider_account_id.clone(),
                    row.account_created_at,
                    row.account_updated_at,
                ) {
                    (
                        Some(account_id),
                        Some(provider),
                        Some(provider_account_id),
                        Some(created_at),
                        Some(updated_at),
                    ) => Some(Account {
                        id: account_id,
                        user_id: first_row.id,
                        provider,
                        provider_account_id,
                        provider_access_token: row.provider_access_token.clone(),
                        provider_refresh_token: row.provider_refresh_token.clone(),
                        provider_token_expires_at: row.provider_token_expires_at,
                        provider_username: row.provider_username.clone(),
                        created_at,
                        updated_at,
                    }),
                    _ => None,
                }
            })
            .collect();

        let settings = first_row.settings_id.and_then(|settings_id| {
            // Only create settings if we have the required timestamps
            match (first_row.settings_created_at, first_row.settings_updated_at) {
                (Some(created_at), Some(updated_at)) => Some(UserSettings {
                    id: settings_id,
                    user_id: first_row.id,
                    stream_status_enabled: first_row.stream_status_enabled,
                    chat_messages_enabled: first_row.chat_messages_enabled,
                    channel_points_enabled: first_row.channel_points_enabled,
                    follows_subs_enabled: first_row.follows_subs_enabled,
                    created_at,
                    updated_at,
                }),
                _ => None,
            }
        });

        Ok(User {
            id: first_row.id,
            email: first_row.email.clone(),
            username: first_row.username.clone(),
            password_hash: first_row.password_hash.clone(),
            role: first_row.role.clone(),
            created_at: first_row.created_at,
            updated_at: first_row.updated_at,
            settings,
            accounts,
        })
    }
    pub async fn add_account(
        &mut self,
        provider: String,
        provider_account_id: String,
        provider_access_token: Option<String>,
        provider_refresh_token: Option<String>,
        provider_token_expires_at: Option<DateTime<Utc>>,
        provider_username: Option<String>,
        pool: &PgPool,
    ) -> Result<&Account, sqlx::Error> {
        let account = sqlx::query_as::<_, Account>(
            r#"
                INSERT INTO accounts (
                    user_id,
                    provider,
                    provider_account_id,
                    provider_access_token,
                    provider_refresh_token,
                    provider_token_expires_at,
                    provider_username
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
                "#,
        )
        .bind(self.id)
        .bind(provider)
        .bind(provider_account_id)
        .bind(provider_access_token)
        .bind(provider_refresh_token)
        .bind(provider_token_expires_at)
        .bind(provider_username)
        .fetch_one(pool)
        .await?;

        self.accounts.push(account);
        Ok(self.accounts.last().unwrap())
    }

    pub async fn get_account_by_provider(&self, provider: &str) -> Option<&Account> {
        self.accounts.iter().find(|a| a.provider == provider)
    }

    pub async fn update_account_tokens(
        &mut self,
        provider: &str,
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
        pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                UPDATE accounts
                SET
                    provider_access_token = $1,
                    provider_refresh_token = $2,
                    provider_token_expires_at = $3,
                    updated_at = CURRENT_TIMESTAMP
                WHERE user_id = $4 AND provider = $5
                "#,
        )
        .bind(access_token.clone())
        .bind(refresh_token.clone())
        .bind(expires_at)
        .bind(self.id)
        .bind(provider)
        .execute(pool)
        .await?;

        // Update the local account data
        if let Some(account) = self.accounts.iter_mut().find(|a| a.provider == provider) {
            account.provider_access_token = Some(access_token);
            account.provider_refresh_token = refresh_token;
            account.provider_token_expires_at = expires_at;
            account.updated_at = Utc::now();
        }

        Ok(())
    }
    /// Gets a user from the databased based on Email
    pub async fn by_email(email: String, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let rows = sqlx::query_as::<_, UserWithSettingsAndAccount>(
            r#"
                SELECT
                    u.*,
                    s.id as settings_id,
                    s.stream_status_enabled,
                    s.chat_messages_enabled,
                    s.channel_points_enabled,
                    s.follows_subs_enabled,
                    s.created_at as settings_created_at,
                    s.updated_at as settings_updated_at,
                    a.id as account_id,
                    a.provider,
                    a.provider_account_id,
                    a.provider_access_token,
                    a.provider_refresh_token,
                    a.provider_token_expires_at,
                    a.provider_username,
                    a.created_at as account_created_at,
                    a.updated_at as account_updated_at
                FROM users u
                LEFT JOIN user_settings s ON s.user_id = u.id
                LEFT JOIN accounts a ON a.user_id = u.id
                WHERE u.email = $1
                "#,
        )
        .bind(email)
        .fetch_all(pool)
        .await?;

        if rows.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        let first_row = &rows[0];

        // Convert the rows into accounts
        let accounts = rows
            .iter()
            .filter_map(|row| {
                match (
                    row.account_id,
                    row.provider.clone(),
                    row.provider_account_id.clone(),
                    row.account_created_at,
                    row.account_updated_at,
                ) {
                    (
                        Some(account_id),
                        Some(provider),
                        Some(provider_account_id),
                        Some(created_at),
                        Some(updated_at),
                    ) => Some(Account {
                        id: account_id,
                        user_id: first_row.id,
                        provider,
                        provider_account_id,
                        provider_access_token: row.provider_access_token.clone(),
                        provider_refresh_token: row.provider_refresh_token.clone(),
                        provider_token_expires_at: row.provider_token_expires_at,
                        provider_username: row.provider_username.clone(),
                        created_at,
                        updated_at,
                    }),
                    _ => None,
                }
            })
            .collect();

        let settings = first_row.settings_id.and_then(|settings_id| {
            // Only create settings if we have the required timestamps
            match (first_row.settings_created_at, first_row.settings_updated_at) {
                (Some(created_at), Some(updated_at)) => Some(UserSettings {
                    id: settings_id,
                    user_id: first_row.id,
                    stream_status_enabled: first_row.stream_status_enabled,
                    chat_messages_enabled: first_row.chat_messages_enabled,
                    channel_points_enabled: first_row.channel_points_enabled,
                    follows_subs_enabled: first_row.follows_subs_enabled,
                    created_at,
                    updated_at,
                }),
                _ => None,
            }
        });

        Ok(User {
            id: first_row.id,
            email: first_row.email.clone(),
            username: first_row.username.clone(),
            password_hash: first_row.password_hash.clone(),
            role: first_row.role.clone(),
            created_at: first_row.created_at,
            updated_at: first_row.updated_at,
            settings,
            accounts,
        })
    }
    /// Gets all users from the database
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let rows = sqlx::query_as::<_, UserWithSettingsAndAccount>(
            r#"
                SELECT
                    u.*,
                    s.id as settings_id,
                    s.stream_status_enabled,
                    s.chat_messages_enabled,
                    s.channel_points_enabled,
                    s.follows_subs_enabled,
                    s.created_at as settings_created_at,
                    s.updated_at as settings_updated_at,
                    a.id as account_id,
                    a.provider,
                    a.provider_account_id,
                    a.provider_access_token,
                    a.provider_refresh_token,
                    a.provider_token_expires_at,
                    a.provider_username,
                    a.created_at as account_created_at,
                    a.updated_at as account_updated_at
                FROM users u
                LEFT JOIN user_settings s ON s.user_id = u.id
                LEFT JOIN accounts a ON a.user_id = u.id
                "#,
        )
        .fetch_all(pool)
        .await?;

        // Group rows by user ID to handle multiple accounts per user
        let mut users_map: std::collections::HashMap<Uuid, User> = std::collections::HashMap::new();

        for row in rows {
            let user_entry = users_map.entry(row.id).or_insert_with(|| {
                let settings = row.settings_id.and_then(|settings_id| {
                    match (row.settings_created_at, row.settings_updated_at) {
                        (Some(created_at), Some(updated_at)) => Some(UserSettings {
                            id: settings_id,
                            user_id: row.id,
                            stream_status_enabled: row.stream_status_enabled,
                            chat_messages_enabled: row.chat_messages_enabled,
                            channel_points_enabled: row.channel_points_enabled,
                            follows_subs_enabled: row.follows_subs_enabled,
                            created_at,
                            updated_at,
                        }),
                        _ => None,
                    }
                });

                User {
                    id: row.id,
                    email: row.email.clone(),
                    username: row.username.clone(),
                    password_hash: row.password_hash.clone(),
                    role: row.role.clone(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    settings,
                    accounts: Vec::new(),
                }
            });

            if let Some(account_id) = row.account_id {
                // Only add account if we have all required fields
                if let (
                    Some(provider),
                    Some(provider_account_id),
                    Some(created_at),
                    Some(updated_at),
                ) = (
                    row.provider.clone(),
                    row.provider_account_id.clone(),
                    row.account_created_at,
                    row.account_updated_at,
                ) {
                    user_entry.accounts.push(Account {
                        id: account_id,
                        user_id: row.id,
                        provider,
                        provider_account_id,
                        provider_access_token: row.provider_access_token,
                        provider_refresh_token: row.provider_refresh_token,
                        provider_token_expires_at: row.provider_token_expires_at,
                        provider_username: row.provider_username,
                        created_at,
                        updated_at,
                    });
                }
            }
        }

        Ok(users_map.into_values().collect())
    }
    /// Updates the user's settings
    pub async fn update_settings(
        &mut self,
        stream_status: bool,
        chat_messages: bool,
        channel_points: bool,
        follows_subs: bool,
        pool: &PgPool,
    ) -> Result<&UserSettings, sqlx::Error> {
        let now = Utc::now();

        let settings = if self.settings.is_some() {
            // Update existing settings
            sqlx::query_as::<_, UserSettings>(
                r#"
                UPDATE user_settings
                SET
                    stream_status_enabled = CASE WHEN $1 THEN $5 ELSE NULL END,
                    chat_messages_enabled = CASE WHEN $2 THEN $5 ELSE NULL END,
                    channel_points_enabled = CASE WHEN $3 THEN $5 ELSE NULL END,
                    follows_subs_enabled = CASE WHEN $4 THEN $5 ELSE NULL END,
                    updated_at = $5
                WHERE user_id = $6
                RETURNING *
                "#,
            )
        } else {
            // Create new settings
            sqlx::query_as::<_, UserSettings>(
                r#"
                INSERT INTO user_settings (
                    user_id,
                    stream_status_enabled,
                    chat_messages_enabled,
                    channel_points_enabled,
                    follows_subs_enabled,
                    created_at,
                    updated_at
                )
                VALUES (
                    $6,
                    CASE WHEN $1 THEN $5 ELSE NULL END,
                    CASE WHEN $2 THEN $5 ELSE NULL END,
                    CASE WHEN $3 THEN $5 ELSE NULL END,
                    CASE WHEN $4 THEN $5 ELSE NULL END,
                    $5,
                    $5
                )
                RETURNING *
                "#,
            )
        }
        .bind(stream_status)
        .bind(chat_messages)
        .bind(channel_points)
        .bind(follows_subs)
        .bind(now)
        .bind(self.id)
        .fetch_one(pool)
        .await?;

        self.settings = Some(settings);
        Ok(self.settings.as_ref().unwrap())
    }
    /// Checks that the raw password matches the expected-to-be-hashed password
    pub fn check_password(&self, raw_password: String) -> Result<(), UserError> {
        let dashed_password = argon2::password_hash::PasswordHash::new(&self.password_hash)
            .map_err(|_| UserError::FailedToHashPassword)?;
        argon2::Argon2::default()
            .verify_password(raw_password.as_bytes(), &dashed_password)
            .map_err(|_| UserError::BadPassword)
    }
    /// Hashes the password for the user
    pub fn hash_password(&mut self) -> Result<&mut Self, UserError> {
        let hashed_password =
            hash_string(&self.password_hash).map_err(|_| UserError::FailedToHashPassword)?;
        self.password_hash = hashed_password;
        Ok(self)
    }
    /// Inserts the user into the database
    pub async fn insert(&self, pool: &PgPool) -> Result<&Self, sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, email, username, password_hash, role) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&self.id)
        .bind(&self.email)
        .bind(&self.username)
        .bind(&self.password_hash)
        .bind(&self.role)
        .execute(pool)
        .await?;

        Ok(self)
    }
}

/// Hash a string using Argon2
pub fn hash_string(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}
