// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        created_by -> Varchar,
        created_on -> Timestamp,
        updated_on -> Timestamp,
        published -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    posts,
);
