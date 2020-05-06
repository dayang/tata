use super::err_str;
use crate::dto::book::*;
use crate::dto::page::*;
use crate::entity::Page;
use crate::schema::page::{dsl as page_dsl, dsl::*};
use diesel::prelude::*;
use diesel::sql_types::Integer;

fn build_catalog(pid: i32, items: &Vec<CatalogItem>) -> Vec<CatalogItem> {
    let mut children = vec![];
    for item in items {
        if item.parent_id == pid {
            let mut child = item.clone();
            child.children = build_catalog(child.id, items);
            children.push(child);
        }
    }

    children.sort_by_key(|i| i.display_order);
    children
}

pub fn get_book_catalog(
    conn: &SqliteConnection,
    the_book_id: i32,
    page_published: Option<bool>,
) -> Result<Vec<CatalogItem>, String> {
    let mut query_statement = page.into_boxed();
    if let Some(is_published) = page_published {
        query_statement = query_statement.filter(published.eq(is_published));
    }
    let page_items = query_statement
        .filter(page_dsl::book_id.eq(the_book_id))
        .select((
            id,
            parent_id,
            display_order,
            title,
            url,
            create_time,
            published,
        ))
        .load::<(i32, i32, i32, String, String, String, bool)>(conn)
        .map_err(err_str)?
        .into_iter()
        .map(|t| CatalogItem {
            id: t.0,
            url: t.4,
            title: t.3,
            parent_id: t.1,
            display_order: t.2,
            create_time: t.5,
            published: t.6,
            children: vec![],
        })
        .collect::<Vec<CatalogItem>>();

    Ok(build_catalog(-1, &page_items))
}

pub fn get_page_by_url(
    conn: &SqliteConnection,
    page_url: String,
    new_hit: bool,
) -> Result<Page, String> {
    let page_find = page
        .filter(url.eq(page_url))
        .first::<Page>(conn)
        .map_err(err_str)?;
    if new_hit {
        let _ = diesel::update(&page_find)
            .set(reads.eq(reads + 1))
            .execute(conn);
    }

    Ok(page_find)
}

pub fn get_page(conn: &SqliteConnection, page_id: i32) -> Result<Page, String> {
    page.find(page_id).first::<Page>(conn).map_err(err_str)
}

pub fn like_page(conn: &SqliteConnection, post_url: String) -> Result<usize, String> {
    diesel::update(page)
        .set(likes.eq(likes + 1))
        .filter(url.eq(post_url))
        .execute(conn)
        .map_err(err_str)
}

pub fn delete(conn: &SqliteConnection, page_id: i32) -> Result<usize, String> {
    diesel::delete(page.find(page_id))
        .execute(conn)
        .map_err(err_str)
}

pub fn create_page_default(
    conn: &SqliteConnection,
    create_page: AddCatalogItem,
) -> Result<CatalogItem, String> {
    use diesel::result::Error;
    conn.transaction::<CatalogItem, Error, _>(|| {
        diesel::insert_into(page)
            .values((
                url.eq(create_page.url),
                title.eq(create_page.title),
                book_id.eq(create_page.book_id),
                parent_id.eq(create_page.parent_id),
                published.eq(false),
                allow_comment.eq(true),
                content.eq("No Content Now"),
                display_order.eq(create_page.display_order),
            ))
            .execute(conn)?;

        no_arg_sql_function!(last_insert_rowid, Integer);
        let insert_id = page.select(last_insert_rowid).first::<i32>(conn)?;

        sort_display_order(
            conn,
            create_page.parent_id,
            insert_id,
            create_page.display_order,
        )?;

        let inserted_page = page.find(insert_id).first::<Page>(conn)?;

        Ok(CatalogItem {
            id: inserted_page.id,
            title: inserted_page.title,
            url: inserted_page.url,
            parent_id: inserted_page.parent_id,
            display_order: inserted_page.display_order,
            create_time: inserted_page.create_time,
            published: inserted_page.published,
            children: vec![],
        })
    })
    .map_err(err_str)
}

pub fn update_page(conn: &SqliteConnection, update_page: UpdatePage) -> Result<usize, String> {
    diesel::update(page.find(update_page.id))
        .set((
            title.eq(update_page.title),
            url.eq(update_page.url),
            content.eq(update_page.content),
            published.eq(update_page.published),
            allow_comment.eq(update_page.allow_comment),
        ))
        .execute(conn)
        .map_err(err_str)
}

fn sort_display_order(
    conn: &SqliteConnection,
    target_parent_id: i32,
    changed_id: i32,
    changed_index: i32,
) -> Result<usize, diesel::result::Error> {
    let mut page_ids = page
        .filter(parent_id.eq(target_parent_id))
        .filter(id.ne(changed_id))
        .select(id)
        .order_by(display_order)
        .load::<i32>(conn)?;
    // 前端传的索引错误
    if changed_index < 0 || changed_index as usize > page_ids.len() {
        return Err(diesel::result::Error::RollbackTransaction);
    }
    page_ids.insert(changed_index as usize, changed_id);

    for (order, pid) in page_ids.iter().enumerate() {
        diesel::update(page.find(pid))
            .set((
                display_order.eq(order as i32),
                parent_id.eq(target_parent_id),
            ))
            .execute(conn)?;
    }

    Ok(0)
}

pub fn change_order(
    conn: &SqliteConnection,
    change_order: MoveCatalogItem,
) -> Result<usize, String> {
    sort_display_order(
        conn,
        change_order.parent_id,
        change_order.id,
        change_order.display_order,
    )
    .map_err(err_str)
}

pub fn delete_book_pages(
    conn: &SqliteConnection,
    delete_book_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(page.filter(book_id.eq(delete_book_id))).execute(conn)
}
