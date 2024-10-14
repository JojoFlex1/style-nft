CREATE TABLE order_history (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,  
    clothing_id INT REFERENCES clothing(id) ON DELETE CASCADE,
    quantity INT NOT NULL,
    total_price FLOAT NOT NULL,
    purchased_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    nft_minted BOOLEAN DEFAULT FALSE  
);

