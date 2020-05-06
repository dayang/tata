use super::err_str;
use crate::dto::pagination::*;
use crate::entity::*;
use crate::schema::book::dsl::*;
use crate::service::pagination::*;
use diesel::prelude::*;

/// no filter for now, because of little amout
pub fn get_books_list(
    conn: &SqliteConnection,
    page: i32,
    limit: Option<i32>,
    published_filter: Option<bool>,
) -> Result<PaginationData<Book>, String> {
    let mut book_list_info = PaginationData::default();

    if page < 1 {
        return Ok(book_list_info);
    }

    let limit = limit.unwrap_or(10);

    let mut query_statement = book.into_boxed();
    if let Some(is_published) = published_filter {
        query_statement = query_statement.filter(published.eq(is_published));
    }
    let paged_books = query_statement
        .paginate(page as i64)
        .per_page(limit as i64)
        .load_and_count_pages::<Book>(conn)
        .map_err(err_str)?;

    book_list_info.total_num = paged_books.2;
    book_list_info.total_pages = paged_books.1;
    book_list_info.per_page = limit;
    book_list_info.curr_page = page;

    book_list_info.page_items = paged_books.0;

    Ok(book_list_info)
}

pub fn get_book(conn: &SqliteConnection, by_id: i32) -> Result<Book, String> {
    book.find(by_id).first::<Book>(conn).map_err(err_str)
}

pub fn get_book_by_name(conn: &SqliteConnection, by_name: String) -> Result<Book, String> {
    book.filter(name.eq(by_name))
        .first::<Book>(conn)
        .map_err(err_str)
}

pub fn create_book(conn: &SqliteConnection, create_field: Book) -> Result<usize, String> {
    diesel::insert_into(book)
        .values((
            name.eq(create_field.name),
            display_text.eq(create_field.display_text),
            cover.eq(create_field.cover),
            published.eq(create_field.published),
            description.eq(create_field.description),
        ))
        .execute(conn)
        .map_err(err_str)
}

pub fn delete(conn: &SqliteConnection, delete_id: i32) -> Result<usize, String> {
    use super::page as page_service;
    conn.transaction::<usize, diesel::result::Error, _>(|| {
        diesel::delete(book.filter(id.eq(delete_id))).execute(conn)?;

        page_service::delete_book_pages(conn, delete_id)
    })
    .map_err(err_str)
}

pub fn update_book(conn: &SqliteConnection, update_field: Book) -> Result<usize, String> {
    diesel::update(book.find(update_field.id))
        .set((
            name.eq(update_field.name),
            display_text.eq(update_field.display_text),
            cover.eq(update_field.cover),
            published.eq(update_field.published),
            description.eq(update_field.description),
        ))
        .execute(conn)
        .map_err(err_str)
}
