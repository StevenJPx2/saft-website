table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        date -> Date,
        body -> Text,
        author -> Varchar,
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
        article_ids -> Nullable<Array<Uuid>>,
        draft_ids -> Nullable<Array<Uuid>>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
