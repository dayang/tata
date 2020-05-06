use crate::dto::book::*;
use crate::dto::page::*;
// use crate::entity::Page;
use crate::service::page as page_service;
use crate::DbConn;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::super::{User, ViewData};

#[get("/pages/edit/<id>")]
pub fn edit_page(_user: User, conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match page_service::get_page(&conn, id) {
        Ok(page) => {
            view_data.add("page", page);
            Ok(Template::render("admin/page/editpage", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/pages/edit", format = "json", data = "<update_page>")]
pub fn edit(_user: User, conn: DbConn, update_page: Json<UpdatePage>) -> JsonValue {
    json!(page_service::update_page(&conn, update_page.0))
}

#[delete("/pages/delete/<id>")]
pub fn delete(_user: User, conn: DbConn, id: i32) -> JsonValue {
    json!(page_service::delete(&conn, id))
}

#[post("/pages/add", format = "json", data = "<create_page>")]
pub fn add(_user: User, conn: DbConn, create_page: Json<AddCatalogItem>) -> JsonValue {
    json!(page_service::create_page_default(&conn, create_page.0))
}

#[post("/pages/changeorder", format = "json", data = "<move_page>")]
pub fn change_order(_user: User, conn: DbConn, move_page: Json<MoveCatalogItem>) -> JsonValue {
    json!(page_service::change_order(&conn, move_page.0))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![edit_page, edit, delete, add, change_order,]
}
