-- Add migration script here
CREATE TABLE
    TeamMembers (
        id INTEGER PRIMARY KEY NOT NULL,
        user_id INTEGER NOT NULL REFERENCES Users (id),
        name TEXT NOT NULL,
        zip_code TEXT NOT NULL,
        title TEXT NULL,
        interests TEXT NULL,
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
    );
