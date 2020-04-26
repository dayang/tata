use crate::entity::Tag;
use crate::service::tag as tag_service;
use crate::DbConn;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::super::{User, ViewData};

#[get("/tags/list")]
pub fn list_page(_user: User) -> Result<Template, Status> {
    Ok(Template::render("admin/tag/tags", json!({})))
}

#[get("/tags/api/all")]
pub fn api_tags_list(_user: User, conn: DbConn) -> JsonValue {
    json!(tag_service::all_tags(&conn))
}

#[get("/tags/edit/<id>")]
pub fn edit_page(_user: User, conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match tag_service::get_tag(&conn, id) {
        Ok(tag) => {
            view_data.add("tag", tag);
            Ok(Template::render("admin/tag/edittag", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/tags/edit", format = "json", data = "<tag>")]
pub fn edit(_user: User, conn: DbConn, tag: Json<Tag>) -> JsonValue {
    json!(tag_service::update(&conn, tag.0))
}

#[get("/tags/add")]
pub fn add_page() -> Result<Template, Status> {
    Ok(Template::render("admin/tag/addtag", json!({})))
}

#[post("/tags/add", format = "json", data = "<tag>")]
pub fn add(_user: User, conn: DbConn, tag: Json<Tag>) -> JsonValue {
    json!(tag_service::add(&conn, tag.0))
}

#[delete("/tags/delete/<id>")]
pub fn delete(_user: User, conn: DbConn, id: i32) -> JsonValue {
    json!(tag_service::delete(&conn, id))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_page,
        edit_page,
        edit,
        add_page,
        add,
        api_tags_list,
        delete,
    ]
}
