// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        avatar -> Text,
        bio -> Text,
        birthday -> Text,
        created_at -> Timestamp,
        email -> Text,
        favorite -> Array<Nullable<Int4>>,
        gender -> Int2,
        nickname -> Text,
        password -> Text,
        phone -> Text,
        position -> Text,
        username -> Text,
    }
}
