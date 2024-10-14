use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use crate::schema;
use diesel::prelude::*;


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = schema::cart)]
pub struct Cart {
    pub id: i32,
    pub user_id: Option<i32>, 
    pub clothing_id: Option<i32>, 
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = schema::clothing)] 
pub struct Clothing {
    pub id: i32,
    pub name: String,
    pub description: Option<String>, 
    pub price: f64, 
    pub is_nft: Option<bool>, 
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = schema::clothing_sizes)]
pub struct ClothingSize {
    pub id: i32,
    pub clothing_id: Option<i32>, 
    pub size: String,
    pub stock: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = schema::nfts)] 
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NFT {
    pub id: i32,
    pub clothing_id: Option<i32>, 
    pub user_id: Option<i32>, 
    pub nft_url: String,
    pub token_id: Option<String>, 
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = schema::order_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderHistory {
    pub id: i32,
    pub user_id: Option<i32>, 
    pub clothing_id: Option<i32>, 
    pub quantity: i32,
    pub total_price: f64,  
    pub nft_minted: Option<bool>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}


