// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        firstname -> Text,
        email -> Text,
        ip -> Text,
    }
}
