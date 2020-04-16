use crate::dto::post::*;
use diesel::prelude::*;
use diesel::dsl::count_star;
use crate::schema::post::{ dsl as post_dsl, dsl::*};
use crate::schema::category::{ dsl as category_dsl, dsl::*};
use crate::schema::posttag::{ dsl as posttag_dsl, dsl::*};
use crate::schema::tag::{ dsl as tag_dsl, dsl::*};
use crate::entity::*;
use super::err_str;
use crate::service::pagination::*;

fn get_post_tags(conn: &SqliteConnection, by_post_id: i32) -> Result<Vec<Tag>, String> {
    tag.inner_join(posttag).filter(post_id.eq(by_post_id)).load::<(Tag, Posttag)>(conn).map_err(err_str).map(|r| {
        r.into_iter().map(|t| t.0).collect()
    })
}

pub fn get_posts_list(conn: &SqliteConnection, page_num: i32, page_index: i32, by_category_name: Option<String>, by_tag_name: Option<String>) -> Result<PostListInfo, String> {
    if page_num < 1 {
        return Err("page_num must greater than 0".into());
    }

    if page_index < 1 {
        return Err("page index must great than 0".into());
    }

    let paged_posts : (Vec<Post>, i64, i64);

    if let Some(by_name) = by_category_name {
        let category_find = category.filter(category_dsl::name.eq(by_name)).first::<Category>(conn).map_err(err_str)?;
        paged_posts = Post::belonging_to(&category_find).filter(published.eq(true)).paginate(page_index as i64).per_page(page_num as i64)
            .load_and_count_pages::<Post>(conn).map_err(err_str)?;
    } else if let Some(by_name) = by_tag_name {
        let tag_find = tag.filter(tag_dsl::name.eq(by_name)).first::<Tag>(conn).map_err(err_str)?;
        paged_posts = Posttag::belonging_to(&tag_find).inner_join(post).filter(published.eq(true)).paginate(page_index as i64).per_page(page_num as i64)
            .load_and_count_pages::<(Posttag, Post)>(conn).map_err(err_str).map(|t| {
                (t.0.into_iter().map(|r| r.1).collect(), t.1, t.2)
            })?;
    } else {
        paged_posts = post.filter(published.eq(true)).paginate(page_index as i64).per_page(page_num as i64)
            .load_and_count_pages::<Post>(conn).map_err(err_str)?;
    }

    let mut post_items = vec![];

    for p in paged_posts.0 {
        post_items.push(PostListItem {
            title: p.title,
            url: p.url,
            summary: p.summary,
            thumbnail: p.thumbnail,
            reads: p.reads,
            create_time: p.create_time,
            tags: match self::get_post_tags(&conn, p.id) {
                Ok(t) => t,
                Err(_) => continue
            },
        });
    }

    let post_list_info = PostListInfo {
        total_num: paged_posts.2 as i32,
        total_pages: paged_posts.1 as i32,
        curr_page: page_index,
        page_items : post_items
    };

    Ok(post_list_info)
}