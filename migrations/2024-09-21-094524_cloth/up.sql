CREATE TABLE clothing (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    price FLOAT NOT NULL,
    stock INT NOT NULL DEFAULT 0,
    size VARCHAR,  
    is_nft BOOLEAN DEFAULT FALSE,  
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

