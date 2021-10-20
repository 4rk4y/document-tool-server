table! {
    elements (id) {
        id -> Nullable<Integer>,
        page_id -> Integer,
        top -> Float,
        right -> Float,
        bottom -> Float,
        left -> Float,
        width -> Float,
        height -> Float,
        align -> Integer,
        data_type -> Integer,
        data -> Text,
    }
}

table! {
    pages (id) {
        id -> Nullable<Integer>,
        title -> Text,
    }
}

joinable!(elements -> pages (page_id));

allow_tables_to_appear_in_same_query!(
    elements,
    pages,
);
