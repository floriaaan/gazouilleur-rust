// @generated automatically by Diesel CLI.

diesel::table! {
    gazouillis (id) {
        id -> Int4,
        username -> Varchar,
        content -> Text,
        created_at -> Timestamp,
    }
}
