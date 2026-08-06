-- Your SQL goes here
ALTER TABLE users
RENAME COLUMN hashed_password TO password;

ALTER TABLE signup_shopkeepers
RENAME COLUMN hashed_password TO password;


