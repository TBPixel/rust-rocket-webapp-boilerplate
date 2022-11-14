-- Add up migration script here
CREATE TABLE profiles (
    user_id VARCHAR PRIMARY KEY NOT NULL,
    email VARCHAR NOT NULL,
    UNIQUE(email),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);