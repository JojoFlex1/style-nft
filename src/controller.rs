use actix_web::{web, HttpResponse, Responder};
use crate::models::{Clothing, NewClothing, CartItem, Purchase}; 
use crate::db;
use diesel::prelude::*;
use crate::schema::clothing::dsl::*;
use crate::schema::cart::dsl::*;
use crate::schema::purchase::dsl::*;
use crate::Pool; 


pub async fn view_clothes(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    match web::block(move || clothing.load::<Clothing>(&conn)).await {
        Ok(clothes) => HttpResponse::Ok().json(clothes),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn add_clothing(
    pool: web::Data<Pool>,
    new_clothing: web::Json<NewClothing>,  
) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let new_clothing = new_clothing.into_inner();

    match web::block(move || {
        diesel::insert_into(clothing)
            .values(&new_clothing)
            .execute(&conn)
    })
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Clothing item added"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn add_to_cart(
    pool: web::Data<Pool>,
    new_cart_item: web::Json<CartItem>,  
) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let new_cart_item = new_cart_item.into_inner();

    match web::block(move || {
        diesel::insert_into(cart)
            .values(&new_cart_item)
            .execute(&conn)
    })
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Item added to cart"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn view_cart(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    match web::block(move || cart.load::<CartItem>(&conn)).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn checkout(
    pool: web::Data<Pool>,
    new_purchase: web::Json<Purchase>,  
) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let new_purchase = new_purchase.into_inner();

    match web::block(move || {
        diesel::insert_into(purchase)
            .values(&new_purchase)
            .execute(&conn)
    })
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Purchase completed"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn view_history(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    match web::block(move || purchase.load::<Purchase>(&conn)).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
