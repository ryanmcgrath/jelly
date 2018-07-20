table! {
    news (id) {
        id -> Int4,
        title -> Text,
        url -> Text,
        added -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Text>,
        email -> Text,
        password -> Text,
        avatar -> Nullable<Text>,
        is_verified -> Bool,
        has_verified_email -> Bool,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    news,
    users,
);
