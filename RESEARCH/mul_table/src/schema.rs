table! {
    storages (id) {
        id -> Int4,
        profile_key -> Varchar,
        post_key -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        profile -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    storages,
    users,
);
