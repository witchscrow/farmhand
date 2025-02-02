-- Remove the role column from users table
ALTER TABLE users
DROP COLUMN role;

-- Remove the user_role enum type
DROP TYPE user_role;
