-- Add up migration script here
CREATE TABLE permissions (
    user_id VARCHAR NOT NULL,
    action VARCHAR NOT NULL,
    resource_id VARCHAR NOT NULL,
    resource_kind VARCHAR NOT NULL,
    PRIMARY KEY (user_id, action, resource_id, resource_kind),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);