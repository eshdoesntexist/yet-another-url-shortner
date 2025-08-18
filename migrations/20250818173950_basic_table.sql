-- Add migration script here
CREATE TABLE shorturls (
    shorturl TEXT PRIMARY KEY NOT NULL,
    longurl  TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);