CREATE TABLE clothing_sizes (
    id SERIAL PRIMARY KEY,
    clothing_id INT REFERENCES clothing(id) ON DELETE CASCADE,
    size VARCHAR NOT NULL,  
    stock INT NOT NULL DEFAULT 0
);
