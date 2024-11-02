-- Add up migration script here
CREATE TABLE IF NOT EXISTS blogposts (
    id CHAR(36) PRIMARY KEY NOT NULL,
    main VARCHAR(255) NOT NULL,
    username VARCHAR(30) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);