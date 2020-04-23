#![allow(dead_code, unused_imports)]
use super::err_str;
use crate::consts::*;
use crate::dto::post::*;
use crate::entity::*;
use crate::schema::category::{dsl as category_dsl, dsl::*};
use crate::schema::post::{dsl as post_dsl, dsl::*};
use crate::schema::posttag::{dsl as posttag_dsl, dsl::*};
use crate::schema::tag::{dsl as tag_dsl, dsl::*};
use crate::service::get_dict_value;
use crate::service::pagination::*;
use crate::util::*;
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::sql_types::{Text, Timestamp};

fn get_post_tags(conn: &SqliteConnection, by_post_id: i32) -> Result<Vec<Tag>, String> {
    tag.inner_join(posttag)
        .filter(post_id.eq(by_post_id))
        .load::<(Tag, Posttag)>(conn)
        .map_err(err_str)
        .map(|r| r.into_iter().map(|t| t.0).collect())
}

pub fn get_posts_list(
    conn: &SqliteConnection,
    page_index: i32,
    by_category_id: Option<i32>,
    by_tag_id: Option<i32>,
    year_month: Option<(i32, i32)>,
) -> Result<PostListInfo, String> {
    let mut post_list_info = PostListInfo::default();

    if page_index < 1 {
        return Ok(post_list_info);
    }

    if let Some((year, month)) = year_month {
        if !valid_year(year) || !valid_month(month) {
            return Ok(post_list_info);
        }
    }

    let page_num = get_dict_value(DICT_POST_PAGE_NUM.into(), &conn)
        .map(|v| v.parse().unwrap_or(DEFAULT_POST_PAGE_NUM))
        .unwrap_or(DEFAULT_POST_PAGE_NUM);

    let paged_posts: (Vec<Post>, i64, i64);

    if let Some(by_id) = by_category_id {
        let category_find = category
            .find(by_id)
            .load::<Category>(conn)
            .map_err(err_str)?;
        paged_posts = Post::belonging_to(&category_find)
            .filter(published.eq(true))
            .paginate(page_index as i64)
            .per_page(page_num as i64)
            .load_and_count_pages::<Post>(conn)
            .map_err(err_str)?;
    } else if let Some(by_id) = by_tag_id {
        let tag_find = tag.find(by_id).load::<Tag>(conn).map_err(err_str)?;
        paged_posts = Posttag::belonging_to(&tag_find)
            .inner_join(post)
            .filter(published.eq(true))
            .paginate(page_index as i64)
            .per_page(page_num as i64)
            .load_and_count_pages::<(Posttag, Post)>(conn)
            .map_err(err_str)
            .map(|t| (t.0.into_iter().map(|r| r.1).collect(), t.1, t.2))?;
    } else if let Some((year, month)) = year_month {
        sql_function!(fn strftime(x: Text, y: Timestamp) -> Text);
        // sql_function!(strftime, Strftime, (x: Text, y: Timestamp) -> Text);
        paged_posts = post
            .filter(published.eq(true))
            .filter(strftime("%Y%m", create_time).eq(format!("{:04}{:02}", year, month)))
            .paginate(page_index as i64)
            .per_page(page_num as i64)
            .load_and_count_pages::<Post>(conn)
            .map_err(err_str)?;
    } else {
        paged_posts = post
            .filter(published.eq(true))
            .paginate(page_index as i64)
            .per_page(page_num as i64)
            .load_and_count_pages::<Post>(conn)
            .map_err(err_str)?;
    }

    post_list_info.total_num = paged_posts.2 as i32;
    post_list_info.total_pages = paged_posts.1 as i32;
    post_list_info.per_page = page_num;
    post_list_info.curr_page = page_index;

    for p in paged_posts.0 {
        post_list_info.page_items.push(PostListItem {
            title: p.title,
            url: p.url,
            summary: p.summary,
            thumbnail: p.thumbnail,
            reads: p.reads,
            create_time: p.create_time,
            tags: match self::get_post_tags(&conn, p.id) {
                Ok(t) => t,
                Err(_) => continue,
            },
        });
    }

    Ok(post_list_info)
}

pub fn get_post_by_url(conn: &SqliteConnection, post_url: String) -> Result<Post, String> {
    post.filter(url.eq(post_url))
        .first::<Post>(conn)
        .map_err(err_str)
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
        // pub raw_content: String,
        html_content: post_find.html_content,
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
