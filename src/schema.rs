// @generated automatically by Diesel CLI.

diesel::table! {
    listings (category) {
        category -> Varchar,
        url -> Varchar,
        timestamp -> Timestamp,
    }
}
