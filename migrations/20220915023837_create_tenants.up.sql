-- Add up migration script here
CREATE TABLE tenants (
    id VARCHAR PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    created_at DATETIME NOT NULL
);
