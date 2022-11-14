-- Add up migration script here
CREATE TABLE users (
    id VARCHAR PRIMARY KEY NOT NULL,
    auth_id VARCHAR NOT NULL,
    created_at DATETIME NOT NULL,
    UNIQUE(auth_id)
);