-- Add up migration script here
CREATE TABLE IF NOT EXISTS roles (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        name TEXT NOT NULL UNIQUE,
        created_at TIMESTAMP DEFAULT now(),
        updated_at TIMESTAMP DEFAULT now()
);

ALTER TABLE users ADD FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE;
INSERT INTO roles (name) VALUES ('Admin'), ('User');
CREATE INDEX IF NOT EXISTS roles_index ON roles (created_at);
