-- Add migration script here
CREATE TABLE Sessions (
    id serial PRIMARY KEY,
    token varchar(255) NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at timestamp NOT NULL
);
