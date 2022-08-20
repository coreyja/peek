-- Add migration script here
CREATE TABLE Sessions (
    id INTEGER PRIMARY KEY NOT NULL,
    token TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL
);
