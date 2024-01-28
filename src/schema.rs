// @generated automatically by Diesel CLI.

diesel::table! {
    listings (id) {
        id -> Int4,
        category -> Varchar,
        url -> Varchar,
        timestamp -> Timestamp,
    }
}
