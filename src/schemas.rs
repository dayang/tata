table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        create_at -> Timestamp,
        published -> Bool,
        category -> Integer,
        visit_count -> Integer,
    }
}