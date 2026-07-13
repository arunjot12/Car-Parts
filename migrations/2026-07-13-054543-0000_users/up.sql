-- Your SQL goes here
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    full_name VARCHAR(255), 
    email VARCHAR(255) UNIQUE, 
    hashed_password VARCHAR(255),
    role Enum('Admin','Staff') NOT NULL DEFAULT 'Staff',
    created_at TimeStamp ,
    updated_at TimeStamp
)
