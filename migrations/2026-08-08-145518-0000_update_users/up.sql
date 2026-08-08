-- Your SQL goes here
ALTER TABLE users
ADD column username VARCHAR(255) UNIQUE AFTER first_name;

ALTER TABLE signup_shopkeepers
ADD column username VARCHAR(255) UNIQUE AFTER first_name;

UPDATE signup_shopkeepers
SET username = first_name
WHERE first_name IS NOT NULL;