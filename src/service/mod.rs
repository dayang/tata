pub mod admin;
pub mod book;
pub mod category;
pub mod comment;
pub mod friendlinks;
pub mod page;
pub mod pagination;
pub mod post;
pub mod tag;

use crate::schema::dict::dsl as dict_dsl;
use diesel::prelude::*;

pub fn err_str<T: ToString>(err: T) -> String {
    err.to_string()
}

pub fn get_dict_value(key: &str, conn: &SqliteConnection) -> Option<String> {
    dict_dsl::dict
        .find(key.to_string())
        .select(dict_dsl::d_value)
        .first::<String>(conn)
        .ok()
}

pub fn set_dict_value<T: ToString>(
    key: &str,
    value: T,
    conn: &SqliteConnection,
) -> Result<usize, String> {
    match dict_dsl::dict
        .find(key.to_string())
        .first::<crate::entity::Dict>(conn)
        .ok()
    {
        Some(dict_item) => diesel::update(&dict_item)
            .set(dict_dsl::d_value.eq(value.to_string()))
            .execute(conn)
            .map_err(err_str),
        None => diesel::insert_into(dict_dsl::dict)
            .values((
                dict_dsl::d_key.eq(key.to_string()),
                dict_dsl::d_value.eq(value.to_string()),
            ))
            .execute(conn)
            .map_err(err_str),
    }
}

pub fn get_dashboard_data(conn: &SqliteConnection) -> Result<crate::dto::DashBoardData, String> {
    use crate::schema::{
        book::dsl as book_dsl, comment::dsl as comment_dsl, friendlink::dsl as friendlink_dsl,
        page::dsl as page_dsl, post::dsl as post_dsl,
    };

    let post_count = post_dsl::post.count().first(conn).map_err(err_str)?;
    let comment_count = comment_dsl::comment.count().first(conn).map_err(err_str)?;
    let book_count = book_dsl::book.count().first(conn).map_err(err_str)?;
    let page_count = page_dsl::page.count().first(conn).map_err(err_str)?;
    let friendlink_count = friendlink_dsl::friendlink
        .count()
        .first(conn)
        .map_err(err_str)?;
    let unread_comments = comment_dsl::comment
        .count()
        .filter(comment_dsl::unread.eq(true))
        .first(conn)
        .map_err(err_str)?;

    Ok(crate::dto::DashBoardData {
        post_count,
        book_count,
        page_count,
        comment_count,
        unread_comments,
        friendlink_count,
    })
}
