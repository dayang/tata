use super::err_str;
use crate::dto::{book::*, pagination::*};
use crate::entity::*;
use crate::schema::book::dsl::*;
use crate::service::pagination::*;
use diesel::prelude::*;

/// no filter for now, because of little amout
pub fn get_books_list(
    conn: &SqliteConnection,
    page: i32,
    limit: Option<i32>,
) -> Result<PaginationData<Book>, String> {
    let mut book_list_info = PaginationData::default();

    if page < 1 {
        return Ok(book_list_info);
    }

    let limit = limit.unwrap_or(10);

    let paged_books = book
        .into_boxed()
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

pub fn create_book(conn: &SqliteConnection, create_field: Book) -> Result<usize, String> {
    Err("no".into())
}

pub fn delete(conn: &SqliteConnection, delete_id: i32) -> Result<usize, String> {
    diesel::delete(book.filter(id.eq(delete_id)))
        .execute(conn)
        .map_err(err_str)
    // TODO delete all pages
}

pub fn update_book(conn: &SqliteConnection, update_field: Book) -> Result<usize, String> {
    Err("no".into())
}
