-- Create enum type for user roles
CREATE TYPE user_role AS ENUM ('admin', 'creator', 'viewer');

-- Add role column to users table with default value of 'viewer'
ALTER TABLE users
ADD COLUMN role user_role NOT NULL DEFAULT 'viewer';
