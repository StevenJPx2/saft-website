table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        date -> Date,
        body -> Text,
        author_id -> Varchar,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        user_name -> Varchar,
        full_name -> Varchar,
        descr -> Text,
        email -> Text,
        password_hash -> Varchar,
        author_ids -> Nullable<Array<Uuid>>,
        draft_ids -> Nullable<Array<Uuid>>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
