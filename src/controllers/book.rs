use crate::service::{book as book_service, page as page_service};
use crate::DbConn;
use rocket::http::Status;
use rocket_contrib::templates::Template;

use super::ViewData;

#[get("/?<page>&<limit>")]
pub fn get_books(page: Option<i32>, limit: Option<i32>, conn: DbConn) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    view_data.add_viewbag("title", "所有Book");
    view_data.add_viewbag("list_title", "所有Book");
    view_data.load_posts_page_meta_data(&conn);
    match book_service::get_books_list(&conn, page.unwrap_or(1), limit, Some(true)) {
        Ok(book_list_info) => {
            view_data.add("book_list_info", book_list_info);
            //println!("{}", view_data.clone().to_json().to_string());
            Ok(Template::render("books", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/book/<name>")]
pub fn book_catalog(conn: DbConn, name: String) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    view_data.load_posts_page_meta_data(&conn);
    match book_service::get_book_by_name(&conn, name) {
        Ok(book) => {
            let id = book.id;
            view_data.add_viewbag("title", book.display_text.clone());
            view_data.add("book", book);

            match page_service::get_book_catalog(&conn, id, Some(true)) {
                Ok(catalog) => {
                    view_data.add("catalog", catalog);
                    Ok(Template::render("bookcatalog", view_data.to_json()))
                }
                Err(_) => Err(Status::InternalServerError),
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_books, book_catalog]
}
