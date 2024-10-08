-- Add up migration script here

-- add note table to store note from shortnotes and diarynotes
CREATE TABLE notes (
    id serial PRIMARY KEY NOT NULL UNIQUE,
    note_id char(16) NOT NULL UNIQUE,
    title VARCHAR(256) NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),
    tags JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- add note content table to store content with history version
CREATE TABLE note_content (
    id serial PRIMARY KEY NOT NULL UNIQUE,
    note_id char(16) references notes(note_id),
    content bytea NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- migrate all data from shortnotes to notes
INSERT INTO notes (note_id, title, user_id, tags, created_at, updated_at)
SELECT snid, title, user_id, tags, created_at, updated_at
FROM shortnotes;

-- migrate all data from diarynotes to notes
INSERT INTO notes (note_id, title, user_id, tags, created_at, updated_at)
SELECT dnid, title, user_id, tags, created_at, updated_at
FROM diarynotes;

-- migrate all content from shortnotes to note_content
INSERT INTO note_content (note_id, content, created_at)
SELECT snid, content, created_at
FROM shortnotes;

-- migrate all content from diarynotes to note_content
INSERT INTO note_content (note_id, content, created_at)
SELECT dnid, content, created_at
FROM diarynotes;
