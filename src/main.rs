use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

mod models;
mod schema;

use models::{Clothing, Cart, NFT}; 

fn connect_wallet() -> Option<String> {
    Some("0x1234...abcd".to_string())
}

async fn mint_nft(clothing_id: &i32, wallet_address: String) -> Result<(), String> {
    println!("Minting NFT for clothing id: {} to wallet: {}", clothing_id, wallet_address);
    Ok(())
}

async fn get_clothes() -> impl Responder {
    HttpResponse::Ok().json(vec!["List of clothing items here"]) // Placeholder
}

async fn get_clothing_by_id(clothing_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(format!("Details of clothing item with id: {}", clothing_id))
}

async fn add_clothes(clothing_data: web::Json<Clothing>) -> impl Responder {
    let clothing = clothing_data.into_inner(); 

    match clothing.is_nft { 
        Some(true) => {
            println!("Adding NFT-exclusive clothing item: {}", clothing.name);
        }
        Some(false) => {
            println!("Adding regular clothing item: {}", clothing.name);
        }
        None => {
            println!("The NFT status of this clothing item is unknown.");
        }
    }

    HttpResponse::Ok().json("Clothing item added successfully")
}

async fn add_to_cart(cart_item: web::Json<Cart>) -> impl Responder {
    HttpResponse::Ok().json("Clothing item added to cart")
}

async fn nft_purchase(nft_data: web::Json<NFT>) -> impl Responder { 
    let wallet_address = connect_wallet();

    if wallet_address.is_none() {
        return HttpResponse::BadRequest().json("Wallet connection failed. Please connect your wallet.");
    }

    
    match mint_nft(&nft_data.clothing_id.unwrap(), wallet_address.unwrap()).await {
        Ok(_) => HttpResponse::Ok().json("NFT purchase successful and minted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("NFT minting failed: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/clothes", web::get().to(get_clothes))
            .route("/clothes/{id}", web::get().to(get_clothing_by_id))
            .route("/clothes", web::post().to(add_clothes))
            .route("/cart", web::post().to(add_to_cart))
            .route("/nft_purchase", web::post().to(nft_purchase)) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
