#![allow(dead_code, unused_imports)]
use super::err_str;
use crate::consts::*;
use crate::dto::{pagination::*, post::*};
use crate::entity::*;
use crate::schema::category::{dsl as category_dsl, dsl::*};
use crate::schema::post::{dsl as post_dsl, dsl::*};
use crate::schema::posttag::{dsl as posttag_dsl, dsl::*};
use crate::schema::tag::{dsl as tag_dsl, dsl::*};
use crate::service::pagination::*;
use crate::service::{category as category_service, get_dict_value};
use crate::util::*;
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text, Timestamp};

pub fn get_post_tags(conn: &SqliteConnection, by_post_id: i32) -> Result<Vec<Tag>, String> {
    tag.inner_join(posttag)
        .filter(post_id.eq(by_post_id))
        .load::<(Tag, Posttag)>(conn)
        .map_err(err_str)
        .map(|r| r.into_iter().map(|t| t.0).collect())
}

pub fn get_posts_list(
    conn: &SqliteConnection,
    page: i32,
    limit: Option<i32>,
    by_category_id: Option<i32>,
    by_tag_id: Option<i32>,
    year_month: Option<(i32, i32)>,
    is_published: Option<bool>,
) -> Result<PaginationData<PostListItem>, String> {
    let mut post_list_info = PaginationData::default();

    if page < 1 {
        return Ok(post_list_info);
    }

    if let Some((year, month)) = year_month {
        if !valid_year(year) || !valid_month(month) {
            return Ok(post_list_info);
        }
    }

    let page_num = limit.unwrap_or_else(|| {
        get_dict_value(DICT_POST_PAGE_NUM.into(), &conn)
            .map(|v| v.parse().unwrap_or(DEFAULT_POST_PAGE_NUM))
            .unwrap_or(DEFAULT_POST_PAGE_NUM)
    });

    let paged_posts: (Vec<Post>, i64, i64);

    let mut paged_posts_query = post.into_boxed();
    if let Some(by_id) = by_tag_id {
        paged_posts_query = paged_posts_query
            .filter(post_dsl::id.eq_any(posttag.filter(tag_id.eq(by_id)).select(post_id)));
    }

    if let Some(by_id) = by_category_id {
        paged_posts_query = paged_posts_query.filter(category_id.eq(by_id));
    }

    if let Some(v) = is_published {
        paged_posts_query = paged_posts_query.filter(published.eq(v));
    }

    if let Some((year, month)) = year_month {
        sql_function!(fn strftime(x: Text, y: Timestamp) -> Text);
        paged_posts_query = paged_posts_query
            .filter(strftime("%Y%m", create_time).eq(format!("{:04}{:02}", year, month)));
    }

    paged_posts = paged_posts_query
        .paginate(page as i64)
        .per_page(page_num as i64)
        .load_and_count_pages::<Post>(conn)
        .map_err(err_str)?;

    post_list_info.total_num = paged_posts.2;
    post_list_info.total_pages = paged_posts.1;
    post_list_info.per_page = page_num;
    post_list_info.curr_page = page;

    for p in paged_posts.0 {
        post_list_info.page_items.push(PostListItem {
            id: p.id,
            title: p.title,
            url: p.url,
            summary: p.summary,
            thumbnail: p.thumbnail,
            reads: p.reads,
            likes: p.likes,
            create_time: p.create_time,
            edit_time: p.edit_time,
            category: match category_service::get_category(&conn, p.category_id) {
                Ok(t) => t,
                Err(_) => continue,
            },
            tags: match self::get_post_tags(&conn, p.id) {
                Ok(t) => t,
                Err(_) => continue,
            },
            published: p.published,
        });
    }

    Ok(post_list_info)
}

pub fn get_post_by_url(conn: &SqliteConnection, post_url: String) -> Result<Post, String> {
    post.filter(url.eq(post_url))
        .first::<Post>(conn)
        .map_err(err_str)
}

/// TODO should be post_detail_admin
pub fn get_post(conn: &SqliteConnection, by_id: i32) -> Result<Post, String> {
    post.find(by_id).first::<Post>(conn).map_err(err_str)
}

pub fn get_post_detail(
    conn: &SqliteConnection,
    post_url: String,
    new_hit: bool,
) -> Result<PostDetail, String> {
    let post_find = get_post_by_url(conn, post_url)?;
    let category_find = category
        .find(post_find.category_id)
        .first::<Category>(conn)
        .map_err(err_str)?;
    let post_tags = get_post_tags(conn, post_find.id).map_err(err_str)?;

    // add reads
    if new_hit {
        let _ = diesel::update(&post_find)
            .set(reads.eq(reads + 1))
            .execute(conn);
    }

    Ok(PostDetail {
        title: post_find.title,
        url: post_find.url,
        content: post_find.content,
        summary: post_find.summary,
        thumbnail: post_find.thumbnail,
        reads: post_find.reads,
        likes: post_find.likes,
        allow_comment: post_find.allow_comment,
        // comment_list_info: comment_service::get_paged_comment(conn, COMMENT_FOR_POST, 1, post_find.id).unwrap_or_default(),
        create_time: post_find.create_time,
        edit_time: post_find.edit_time,
        tags: post_tags,
        category: category_find,
    })
}

pub fn like_post(conn: &SqliteConnection, post_url: String) -> Result<usize, String> {
    diesel::update(post)
        .set(likes.eq(likes + 1))
        .filter(url.eq(post_url))
        .execute(conn)
        .map_err(err_str)
}

pub fn archive_posts(conn: &SqliteConnection) -> Result<Vec<PostYearArchive>, String> {
    use std::collections::HashMap;
    let times = post
        .select(create_time)
        .load::<NaiveDateTime>(conn)
        .map_err(err_str)?;
    let hash_map = times.into_iter().fold(
        HashMap::new(),
        |mut map: HashMap<String, HashMap<String, i32>>, timestamp| {
            let date = timestamp.date();
            let year = format!("{:04}", date.year());
            let month = format!("{:02}", date.month());

            let months = map.entry(year).or_insert(HashMap::new());
            months.entry(month).and_modify(|n| *n += 1).or_insert(1);
            map
        },
    );

    let mut year_archive = hash_map
        .into_iter()
        .map(|(k, v)| {
            let mut months = v
                .into_iter()
                .map(|(m, n)| PostMonthArchive { month: m, num: n })
                .collect::<Vec<PostMonthArchive>>();

            months.sort();

            PostYearArchive {
                year: k,
                months: months,
            }
        })
        .collect::<Vec<PostYearArchive>>();

    year_archive.sort_by_key(|a| std::cmp::Reverse(a.year.clone()));

    Ok(year_archive)
}

pub fn create_post(
    conn: &SqliteConnection,
    create_feild: CreateOrUpdatePost,
) -> Result<usize, String> {
    use diesel::result::Error;
    conn.transaction::<usize, Error, _>(|| {
        diesel::insert_into(post)
            .values((
                title.eq(create_feild.title),
                url.eq(create_feild.url),
                summary.eq(create_feild.summary),
                thumbnail.eq(create_feild.thumbnail),
                content.eq(create_feild.content),
                allow_comment.eq(create_feild.allow_comment),
                published.eq(create_feild.published),
                category_id.eq(create_feild.category_id),
            ))
            .execute(conn)?;

        no_arg_sql_function!(last_insert_rowid, Integer);
        let insert_id = post.select(last_insert_rowid).first::<i32>(conn)?;

        let mut post_tags = vec![];
        for tid in create_feild.tags {
            post_tags.push((tag_id.eq(tid), post_id.eq(insert_id)));
        }
        let n = diesel::insert_into(posttag)
            .values(&post_tags)
            .execute(conn)?;
        Ok(n)
    })
    .map_err(err_str)
}

pub fn update_post(
    conn: &SqliteConnection,
    update_field: CreateOrUpdatePost,
) -> Result<usize, String> {
    use diesel::dsl::now;
    use diesel::result::Error;
    conn.transaction::<usize, Error, _>(|| {
        diesel::update(post)
            .filter(post_dsl::id.eq(update_field.id))
            .set((
                title.eq(update_field.title),
                url.eq(update_field.url),
                summary.eq(update_field.summary),
                thumbnail.eq(update_field.thumbnail),
                content.eq(update_field.content),
                allow_comment.eq(update_field.allow_comment),
                published.eq(update_field.published),
                category_id.eq(update_field.category_id),
                edit_time.eq(now),
            ))
            .execute(conn)?;

        diesel::delete(posttag.filter(posttag_dsl::post_id.eq(update_field.id))).execute(conn)?;

        let mut post_tags = vec![];
        for tid in update_field.tags {
            post_tags.push((tag_id.eq(tid), post_id.eq(update_field.id)));
        }
        let n = diesel::insert_into(posttag)
            .values(&post_tags)
            .execute(conn)?;
        Ok(n)
    })
    .map_err(err_str)
}

pub fn delete(conn: &SqliteConnection, delete_id: i32) -> Result<usize, String> {
    diesel::delete(post.filter(post_dsl::id.eq(delete_id)))
        .execute(conn)
        .map_err(err_str)
}
