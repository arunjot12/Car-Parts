-- Your SQL goes here
CREATE TABLE signup_shopkeepers (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(255), 
    email VARCHAR(255) UNIQUE, 
    hashed_password VARCHAR(255),
    phone_number CHAR(10),
    shop_name VARCHAR (255),
    shop_address VARCHAR(255),
    city VARCHAR(255),
    created_at TimeStamp ,
    updated_at TimeStamp
)
