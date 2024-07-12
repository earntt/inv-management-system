-- Add down migration script here
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_role_id_fkey;
DROP TABLE IF EXISTS roles;
