table! {
    solutions (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        descr -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        full_name -> Varchar,
        email -> Text,
        pass -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    solutions,
    users,
);
