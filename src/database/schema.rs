// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Integer,
        name -> Text,
        colour -> Text,
        link -> Text,
        assigned -> Nullable<Text>,
    }
}
