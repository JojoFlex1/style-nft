// @generated automatically by Diesel CLI.

diesel::table! {
    cart (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        clothing_id -> Nullable<Int4>,
        quantity -> Int4,
        added_at -> Timestamp,
    }
}

diesel::table! {
    clothing (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Float8,
        stock -> Int4,
        size -> Nullable<Varchar>,
        is_nft -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    clothing_sizes (id) {
        id -> Int4,
        clothing_id -> Nullable<Int4>,
        size -> Varchar,
        stock -> Int4,
    }
}

diesel::table! {
    nfts (id) {
        id -> Int4,
        clothing_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        nft_url -> Varchar,
        created_at -> Timestamp,
        token_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    order_history (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        clothing_id -> Nullable<Int4>,
        quantity -> Int4,
        total_price -> Float8,
        purchased_at -> Timestamp,
        nft_minted -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(cart -> clothing (clothing_id));
diesel::joinable!(cart -> users (user_id));
diesel::joinable!(clothing_sizes -> clothing (clothing_id));
diesel::joinable!(nfts -> clothing (clothing_id));
diesel::joinable!(nfts -> users (user_id));
diesel::joinable!(order_history -> clothing (clothing_id));
diesel::joinable!(order_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart,
    clothing,
    clothing_sizes,
    nfts,
    order_history,
    users,
);
