-- Add up migration script here
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL, -- 'twitch', etc.
    provider_account_id VARCHAR(255) NOT NULL, -- external ID from the provider
    provider_access_token TEXT,
    provider_refresh_token TEXT,
    provider_token_expires_at TIMESTAMP,
    provider_username VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(provider, provider_account_id)
);

-- Index for faster lookups by user_id
CREATE INDEX idx_accounts_user_id ON accounts(user_id);
