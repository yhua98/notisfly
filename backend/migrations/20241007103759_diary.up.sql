-- Add up migration script here

-- Create diary table
CREATE TABLE diarynotes (
    id SERIAL PRIMARY KEY NOT NULL,
    dnid VARCHAR(8) NOT NULL UNIQUE,
    title VARCHAR(256) NOT NULL,
    content bytea NOT NULL DEFAULT '',
    user_id INTEGER NOT NULL REFERENCES users(id),
    tags JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);