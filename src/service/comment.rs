use super::err_str;
use crate::consts::*;
use crate::dto::{comment::*, pagination::*};
use crate::entity::Comment as CommentEntity;
use crate::schema::comment::dsl::*;
use crate::service::dict::get_dict_value;
use crate::service::pagination::*;
use diesel::prelude::*;

pub fn get_comment(
    conn: &SqliteConnection,
    comment_id: i32,
    make_read: bool,
) -> Result<(CommentEntity, String), String> {
    if make_read {
        let _res = diesel::update(comment.filter(id.eq(comment_id)))
            .set(unread.eq(false))
            .execute(conn);
    }

    let comment_entity = comment
        .find(comment_id)
        .first::<CommentEntity>(conn)
        .map_err(err_str)?;

    use crate::schema::page::dsl as page_dsl;
    use crate::schema::post::dsl as post_dsl;
    let for_url;
    if comment_entity.comment_type == COMMENT_FOR_POST {
        for_url = post_dsl::post
            .filter(post_dsl::id.eq(comment_entity.foreign_id))
            .select(post_dsl::url)
            .first::<String>(conn)
            .map_err(err_str)?
    } else if comment_entity.comment_type == COMMENT_FOR_PAGE {
        for_url = page_dsl::page
            .filter(page_dsl::id.eq(comment_entity.foreign_id))
            .select(page_dsl::url)
            .first::<String>(conn)
            .map_err(err_str)?
    } else {
        for_url = "no url".to_string();
    }

    Ok((comment_entity, for_url))
}

pub fn update(
    conn: &SqliteConnection,
    update_field: UpdateCommentRequest,
) -> Result<usize, String> {
    use diesel::dsl::now;
    diesel::update(comment.filter(id.eq(update_field.id)))
        .set((
            reply.eq(update_field.reply),
            show.eq(update_field.show),
            reply_time.eq(now),
        ))
        .execute(conn)
        .map_err(err_str)
}

pub fn delete(conn: &SqliteConnection, delete_id: i32) -> Result<usize, String> {
    diesel::delete(comment.filter(id.eq(delete_id)))
        .execute(conn)
        .map_err(err_str)
}

pub fn get_paged_comment(
    conn: &SqliteConnection,
    comment_for: i32,
    page_index: i32,
    master_id: i32,
) -> Result<PaginationData<CommentListItem>, String> {
    let page_num = get_dict_value(DICT_COMMENT_PAGE_NUM.into(), &conn)
        .map(|v| v.parse().unwrap_or(DEFAULT_COMMENT_PAGE_NUM))
        .unwrap_or(DEFAULT_COMMENT_PAGE_NUM);
    let (comments, total_pages, total_num) = comment
        .filter(foreign_id.eq(master_id))
        .filter(comment_type.eq(comment_for))
        .filter(show.eq(true))
        .paginate(page_index as i64)
        .per_page(page_num as i64)
        .load_and_count_pages::<CommentEntity>(conn)
        .map_err(err_str)?;

    Ok(PaginationData {
        total_num,
        total_pages,
        per_page: page_num,
        curr_page: page_index,
        page_items: comments
            .into_iter()
            .map(|detail| CommentListItem {
                user_name: detail.user_name,
                avatar: format!("{:x}", md5::compute(detail.email)),
                content: detail.content,
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
        content.eq(request.content),
        show.eq(true),
        foreign_id.eq(for_id),
        comment_type.eq(for_type),
    ));

    // println!("{}", debug_query::<Sqlite, _>(&query));
    query.execute(conn).map_err(err_str)
}

pub fn get_paged_comment_admin(
    conn: &SqliteConnection,
    comment_for: Option<i32>,
    master_id: Option<i32>,
    page: i32,
    limit: i32,
    unread_filter: Option<bool>,
) -> Result<PaginationData<CommentListItemAdmin>, String> {
    if page < 1 {
        return Err("page must greater than 0".into());
    }

    if limit < 1 {
        return Err("limit must greater than 0".into());
    }

    let mut query = comment.into_boxed();
    if let Some(v) = unread_filter {
        query = query.filter(unread.eq(v));
    }

    if let Some(for_type) = comment_for {
        query = query.filter(comment_type.eq(for_type));
        if for_type != COMMENT_FOR_MESSAGE_BORD {
            if let Some(for_id) = master_id {
                query = query.filter(foreign_id.eq(for_id));
            }
        }
    }

    let (comments, total_pages, total_num) = query
        .paginate(page as i64)
        .per_page(limit as i64)
        .load_and_count_pages::<CommentEntity>(conn)
        .map_err(err_str)?;

    Ok(PaginationData {
        total_num,
        total_pages,
        per_page: limit,
        curr_page: page,
        page_items: comments
            .into_iter()
            .map(|detail| CommentListItemAdmin {
                id: detail.id,
                user_name: detail.user_name,
                email: detail.email,
                comment_time: detail.comment_time,
                unread: detail.unread,
                show: detail.show,
                comment_type: detail.comment_type,
            })
            .collect(),
    })
}
