use crate::entity::Book;
use crate::service::{book as book_service, page as page_service};
use crate::DbConn;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::super::{User, ViewData};

#[get("/books/list")]
pub fn list_page(_user: User) -> Result<Template, Status> {
    Ok(Template::render("admin/book/books", json!({})))
}

#[get("/books/api/list?<page>&<limit>")]
pub fn api_books_list(
    _user: User,
    conn: DbConn,
    page: Option<i32>,
    limit: Option<i32>,
) -> JsonValue {
    json!(book_service::get_books_list(
        &conn,
        page.unwrap_or(1),
        limit,
        None
    ))
}

#[get("/books/edit/<id>")]
pub fn edit_page(_user: User, conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match book_service::get_book(&conn, id) {
        Ok(book) => {
            view_data.add("book", book);
            Ok(Template::render("admin/book/editbook", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/books/edit", format = "json", data = "<update_book>")]
pub fn edit(_user: User, conn: DbConn, update_book: Json<Book>) -> JsonValue {
    json!(book_service::update_book(&conn, update_book.0))
}

#[delete("/books/delete/<id>")]
pub fn delete(_user: User, conn: DbConn, id: i32) -> JsonValue {
    json!(book_service::delete(&conn, id))
}

#[get("/books/add")]
pub fn add_page() -> Result<Template, Status> {
    Ok(Template::render("admin/book/addbook", json!({})))
}

#[post("/books/add", format = "json", data = "<create_book>")]
pub fn add(_user: User, conn: DbConn, create_book: Json<Book>) -> JsonValue {
    json!(book_service::create_book(&conn, create_book.0))
}

#[get("/books/edit_catalog/<id>")]
pub fn edit_catalog_page(_user: User, conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match book_service::get_book(&conn, id) {
        Ok(book) => {
            view_data.add("book", book);
            Ok(Template::render(
                "admin/book/editbookcatalog",
                view_data.to_json(),
            ))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/books/catalog/<id>")]
pub fn get_book_catalog(_user: User, conn: DbConn, id: i32) -> JsonValue {
    json!(page_service::get_book_catalog(&conn, id, None))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_page,
        edit_page,
        edit,
        api_books_list,
        delete,
        add_page,
        add,
        edit_catalog_page,
        get_book_catalog
    ]
}
