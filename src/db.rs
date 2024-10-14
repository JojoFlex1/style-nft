use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel; 

pub mod schema; 

pub fn establish_connection() -> PgConnection {
    dotenv().ok(); 
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use crate::models::{Clothing, NewClothing};

pub fn create_clothing(conn: &PgConnection, new_clothing: NewClothing) -> Clothing {
    use crate::schema::clothing;

    diesel::insert_into(clothing::table)
        .values(&new_clothing)
        .get_result(conn)
        .expect("Error saving new clothing item")
}

pub fn get_all_clothing(conn: &PgConnection) -> Vec<Clothing> {
    use crate::schema::clothing::dsl::*;

    clothing
        .load::<Clothing>(conn)
        .expect("Error loading clothing")
}

pub fn update_clothing(conn: &PgConnection, id: i32, updated_clothing: NewClothing) -> Clothing {
    use crate::schema::clothing::dsl::*;

    diesel::update(clothing.find(id))
        .set(&updated_clothing)
        .get_result(conn)
        .expect("Error updating clothing item")
}

pub fn delete_clothing(conn: &PgConnection, id: i32) -> usize {
    use crate::schema::clothing::dsl::*;

    diesel::delete(clothing.find(id))
        .execute(conn)
        .expect("Error deleting clothing item")
}

// db for ClothingSize
use crate::models::{ClothingSize, NewClothingSize};

pub fn create_clothing_size(conn: &PgConnection, new_size: NewClothingSize) -> ClothingSize {
    use crate::schema::clothing_sizes;

    diesel::insert_into(clothing_sizes::table)
        .values(&new_size)
        .get_result(conn)
        .expect("Error saving new clothing size")
}

pub fn get_all_clothing_sizes(conn: &PgConnection) -> Vec<ClothingSize> {
    use crate::schema::clothing_sizes::dsl::*;

    clothing_sizes
        .load::<ClothingSize>(conn)
        .expect("Error loading clothing sizes")
}

pub fn update_clothing_size(conn: &PgConnection, id: i32, updated_size: NewClothingSize) -> ClothingSize {
    use crate::schema::clothing_sizes::dsl::*;

    diesel::update(clothing_sizes.find(id))
        .set(&updated_size)
        .get_result(conn)
        .expect("Error updating clothing size")
}

pub fn delete_clothing_size(conn: &PgConnection, id: i32) -> usize {
    use crate::schema::clothing_sizes::dsl::*;

    diesel::delete(clothing_sizes.find(id))
        .execute(conn)
        .expect("Error deleting clothing size")
}

// db for NFT clothes
use crate::models::{NFT, NewNFT};

pub fn create_nft(conn: &PgConnection, new_nft: NewNFT) -> NFT {
    use crate::schema::nfts;

    diesel::insert_into(nfts::table)
        .values(&new_nft)
        .get_result(conn)
        .expect("Error saving new NFT")
}

pub fn get_all_nfts(conn: &PgConnection) -> Vec<NFT> {
    use crate::schema::nfts::dsl::*;

    nfts
        .load::<NFT>(conn)
        .expect("Error loading NFTs")
}

pub fn update_nft(conn: &PgConnection, id: i32, updated_nft: NewNFT) -> NFT {
    use crate::schema::nfts::dsl::*;

    diesel::update(nfts.find(id))
        .set(&updated_nft)
        .get_result(conn)
        .expect("Error updating NFT")
}

pub fn delete_nft(conn: &PgConnection, id: i32) -> usize {
    use crate::schema::nfts::dsl::*;

    diesel::delete(nfts.find(id))
        .execute(conn)
        .expect("Error deleting NFT")
}
