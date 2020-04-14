table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        create_at -> Timestamp,
        published -> Bool,
        category_id -> Integer,
        visit_count -> Integer,
    }
}

table! {
    categorys (id) {
        id -> Integer,
        label -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    categorys,
);
