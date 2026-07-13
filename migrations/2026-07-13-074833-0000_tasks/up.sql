-- Your SQL goes here
CREATE TABLE task(
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255),
    description TEXT,
    status enum('todo','in_progress','complete'),
    priority enum('low','medium','high'),
    created_by_id VARCHAR(255),
    assigned_to_id VARCHAR(255), 
    created_at TIMESTAMP,

    FOREIGN KEY (created_by_id) REFERENCES users(email),
    FOREIGN KEY (assigned_to_id) REFERENCES users(email)
)