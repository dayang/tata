table! {
    book (id) {
        id -> Integer,
        name -> Text,
        display_text -> Text,
        create_time -> Timestamp,
    }
}

table! {
    category (id) {
        id -> Integer,
        name -> Text,
        display_text -> Text,
    }
}

table! {
    comment (id) {
        id -> Integer,
        user_name -> Text,
        email -> Text,
        content -> Text,
        comment_time -> Timestamp,
        reply -> Nullable<Text>,
        reply_time -> Timestamp,
        show -> Bool,
        post_id -> Integer,
        user_agent -> Nullable<Text>,
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
        remark -> Nullable<Text>,
    }
}

table! {
    logininfo (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        username -> Text,
        login_time -> Timestamp,
        is_success -> Bool,
        ip -> Nullable<Text>,
        mac -> Nullable<Text>,
        user_agent -> Nullable<Text>,
    }
}

table! {
    post (id) {
        id -> Integer,
        title -> Text,
        url -> Text,
        raw_content -> Text,
        html_content -> Text,
        summary -> Text,
        thumbnail -> Text,
        reads -> Integer,
        likes -> Integer,
        allow_comment -> Bool,
        create_time -> Timestamp,
        edit_time -> Timestamp,
        category_id -> Nullable<Integer>,
        post_type -> Integer,
        parent_id -> Nullable<Integer>,
        book_id -> Nullable<Integer>,
        display_order -> Nullable<Integer>,
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
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Text,
        nick_name -> Nullable<Text>,
        description -> Nullable<Text>,
        password -> Text,
        avator -> Nullable<Text>,
        email -> Nullable<Text>,
        notify_comment -> Bool,
        notify_type -> Nullable<Integer>,
        notify_email -> Nullable<Text>,
        session_period -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    book,
    category,
    comment,
    dict,
    friendlink,
    logininfo,
    post,
    posttag,
    tag,
    user,
);
