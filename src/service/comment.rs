use super::err_str;
use crate::consts::*;
use crate::dto::comment::*;
use crate::entity::Comment as CommentEntity;
use crate::schema::comment::dsl::*;
use crate::service::get_dict_value;
use crate::service::pagination::*;
// use diesel::debug_query;
use diesel::prelude::*;
// use diesel::sqlite::Sqlite;

pub fn get_paged_comment(
    conn: &SqliteConnection,
    comment_for: i32,
    page_index: i32,
    master_id: i32,
) -> Result<CommentListInfo, String> {
    let page_num = get_dict_value(DICT_COMMENT_PAGE_NUM.into(), &conn)
        .map(|v| v.parse().unwrap_or(DEFAULT_COMMENT_PAGE_NUM))
        .unwrap_or(DEFAULT_COMMENT_PAGE_NUM);
    let (comments, total_pages, total_num) = comment
        .filter(foreign_id.eq(master_id))
        .filter(comment_type.eq(comment_for))
        .paginate(page_index as i64)
        .per_page(page_num as i64)
        .load_and_count_pages::<CommentEntity>(conn)
        .map_err(err_str)?;

    Ok(CommentListInfo {
        total_num,
        total_pages,
        per_page: page_num,
        curr_page: page_index,
        page_items: comments
            .into_iter()
            .map(|detail| CommentListItem {
                user_name: detail.user_name,
                avatar: format!("{:x}", md5::compute(detail.email)),
                raw_content: detail.raw_content,
                html_content: detail.html_content,
                comment_time: detail.comment_time,
                reply: detail.reply,
                reply_time: detail.reply_time,
            })
            .collect(),
    })
}

pub fn new_comment(
    conn: &SqliteConnection,
    request: CommentRequest,
    for_id: i32,
    for_type: i32,
) -> Result<usize, String> {
    let query = diesel::insert_into(comment).values((
        user_name.eq(request.user_name),
        email.eq(request.email),
        raw_content.eq(request.raw_content),
        html_content.eq(request.html_content),
        show.eq(true),
        foreign_id.eq(for_id),
        comment_type.eq(for_type),
    ));

    // println!("{}", debug_query::<Sqlite, _>(&query));
    query.execute(conn).map_err(err_str)
}

/// for admin
pub fn reply_comment(conn: &SqliteConnection, reply_content: String, comment_id: i32) -> bool {
    true
}
