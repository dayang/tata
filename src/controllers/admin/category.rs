
use crate::entity::Category;
use crate::service::category as category_service;
use crate::DbConn;
use rocket::http::{Status};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::super::{User, ViewData};

#[get("/categorys/list")]
pub fn list_page(_user: User) -> Result<Template, Status> {
    Ok(Template::render("admin/category/categorys", json!({})))
}

#[get("/categorys/api/all")]
pub fn api_categorys_list(_user: User, conn: DbConn) -> JsonValue {
    json!(category_service::all_categorys(&conn))
}

#[get("/categorys/edit/<id>")]
pub fn edit_page(_user: User, conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match category_service::get_category(&conn, id) {
        Ok(category) => {
            view_data.add("category", category);
            Ok(Template::render("admin/category/editcategory", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/categorys/edit", format = "json", data = "<category>")]
pub fn edit(_user: User, conn: DbConn, category: Json<Category>) -> JsonValue {
    json!(category_service::update(&conn, category.0))
}

#[get("/categorys/add")]
pub fn add_page() -> Result<Template, Status> {
    Ok(Template::render("admin/category/addcategory", json!({})))
}

#[post("/categorys/add", format = "json", data = "<category>")]
pub fn add(_user: User, conn: DbConn, category: Json<Category>) -> JsonValue {
    json!(category_service::add(&conn, category.0))
}

#[delete("/categorys/delete/<id>")]
pub fn delete(_user: User, conn: DbConn, id: i32) -> JsonValue {
    json!(category_service::delete(&conn, id))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_page,
        edit_page, 
        edit,
        add_page,
        add,
        api_categorys_list,
        delete,
    ]
}
