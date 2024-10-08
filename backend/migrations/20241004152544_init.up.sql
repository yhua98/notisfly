-- Add up migration script here

-- Create users table
create table users (
    id serial primary key not null,
    name varchar(64) not null,
    email varchar(64) not null,
    password varchar(64) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

-- Create shortnotes table
create table shortnotes (
    id serial primary key not null,
    snid varchar(16) not null,
    title varchar(256) not null,
    content bytea not null default '',
    user_id integer not null references users(id),
    tags jsonb not null default '[]',
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
);