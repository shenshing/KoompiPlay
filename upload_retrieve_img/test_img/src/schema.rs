table! {
    files (id) {
        id -> Int4,
        images -> Nullable<Array<Text>>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        profile -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    users,
);
