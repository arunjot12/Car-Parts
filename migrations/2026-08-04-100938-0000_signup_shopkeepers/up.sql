-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(255), 
    email VARCHAR(255) UNIQUE, 
    hashed_password VARCHAR(255),
    phone_number CHAR(10),
    created_at TimeStamp ,
    updated_at TimeStamp
)
