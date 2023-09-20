IF EXISTS(backend_db)
DROP DATABASE backend_db;

CREATE DATABASE backend_db;

CREATE TABLE pastes (
    id int primary key,
    content text,
    lang varchar(32),
);
