-- Add up migration script here
CREATE TABLE IF NOT EXISTS pasta (
    id int PRIMARY KEY NOT NULL,
    lang varchar(32),
    text text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
