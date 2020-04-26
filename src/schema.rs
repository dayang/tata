table! {
    book (id) {
        id -> Integer,
        name -> Text,
        display_text -> Text,
        description -> Text,
        cover -> Text,
        published -> Bool,
        create_time -> Timestamp,
    }
}

table! {
    category (id) {
        id -> Integer,
        name -> Text,
        display_text -> Text,
        remark -> Text,
    }
}

table! {
    comment (id) {
        id -> Integer,
        user_name -> Text,
        email -> Text,
        content -> Text,
        comment_time -> Timestamp,
        reply -> Text,
        reply_time -> Timestamp,
        show -> Bool,
        foreign_id -> Integer,
        comment_type -> Integer,
        unread -> Bool,
        user_agent -> Text,
    }
}

table! {
    dict (d_key) {
        d_key -> Text,
        d_value -> Text,
    }
}

table! {
    friendlink (id) {
        id -> Integer,
        display_text -> Text,
        link -> Text,
        show -> Bool,
        remark -> Text,
    }
}

table! {
    logininfo (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        username -> Text,
        login_time -> Timestamp,
        is_success -> Bool,
        ip -> Text,
        mac -> Text,
        user_agent -> Text,
    }
}

table! {
    page (id) {
        id -> Integer,
        title -> Text,
        url -> Text,
        content -> Text,
        reads -> Integer,
        likes -> Integer,
        allow_comment -> Bool,
        published -> Bool,
        create_time -> Timestamp,
        edit_time -> Timestamp,
        parent_id -> Integer,
        book_id -> Integer,
        display_order -> Integer,
    }
}

table! {
    post (id) {
        id -> Integer,
        title -> Text,
        url -> Text,
        content -> Text,
        summary -> Text,
        thumbnail -> Text,
        reads -> Integer,
        likes -> Integer,
        allow_comment -> Bool,
        published -> Bool,
        create_time -> Timestamp,
        edit_time -> Timestamp,
        category_id -> Integer,
    }
}

table! {
    posttag (id) {
        id -> Integer,
        post_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    tag (id) {
        id -> Integer,
        name -> Text,
        display_text -> Text,
        remark -> Text,
        weight -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Text,
        nick_name -> Text,
        description -> Text,
        password -> Text,
        avator -> Text,
        email -> Text,
        notify_comment -> Bool,
        notify_type -> Integer,
        notify_email -> Text,
        session_period -> Integer,
    }
}

joinable!(page -> book (book_id));
joinable!(post -> category (category_id));
joinable!(posttag -> post (post_id));
joinable!(posttag -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    book, category, comment, dict, friendlink, logininfo, page, post, posttag, tag, user,
);
