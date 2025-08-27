-- Add migration script here
CREATE TABLE users (
    id INT PRIMARY KEY  NOT NULL,
    email  TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);