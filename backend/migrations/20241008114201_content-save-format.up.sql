-- Add up migration script here

-- add note table to store note from shortnotes and diarynotes
CREATE TABLE notes (
    id serial PRIMARY KEY NOT NULL UNIQUE,
    note_id char(16) NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),

    note_type VARCHAR(16) NOT NULL CHECK (note_type IN ('diary', 'note')) DEFAULT 'note',
    note_version INTEGER NOT NULL DEFAULT 1,
    is_latest BOOLEAN NOT NULL DEFAULT FALSE,

    title VARCHAR(256) NOT NULL,
    tags JSONB NOT NULL DEFAULT '[]',
    content bytea NOT NULL DEFAULT '',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- migrate all data from shortnotes to notes
INSERT INTO notes (note_id, title, user_id, tags, created_at, content, note_version, is_latest)
SELECT snid, title, user_id, tags, created_at, content, 1, TRUE
FROM shortnotes;

-- migrate all data from diarynotes to notes
INSERT INTO notes (note_id, title, user_id, tags, created_at, content, note_type, note_version, is_latest)
SELECT dnid, title, user_id, tags, created_at, content, 'diary', 1, TRUE
FROM diarynotes;
