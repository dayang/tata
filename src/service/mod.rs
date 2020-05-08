pub mod admin;
pub mod book;
pub mod category;
pub mod comment;
pub mod dict;
pub mod friendlinks;
pub mod page;
pub mod pagination;
pub mod post;
pub mod tag;

use diesel::prelude::*;

pub fn err_str<T: ToString>(err: T) -> String {
    err.to_string()
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
