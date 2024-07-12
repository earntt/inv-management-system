-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        name TEXT NOT NULL UNIQUE,
        email TEXT NOT NULL UNIQUE,
        hash TEXT NOT NULL,
        address TEXT,
        role_id UUID NOT NULL,
        created_at TIMESTAMP DEFAULT now(),
        updated_at TIMESTAMP DEFAULT now()
);

CREATE INDEX IF NOT EXISTS users_index ON users (created_at, email);
