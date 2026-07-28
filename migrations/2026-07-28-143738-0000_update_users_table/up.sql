-- Your SQL goes here
ALTER table users
MODIFY email VARCHAR(255) NOT NULL;

ALTER TABLE users
MODIFY role ENUM('ADMIN', 'STAFF', 'SHOPKEEPER', 'CUSTOMER') NOT NULL;

UPDATE users
SET role = 'CUSTOMER' where role = 'Staff';

ALTER TABLE users
MODIFY hashed_password VARCHAR(255) NOT NULL;
