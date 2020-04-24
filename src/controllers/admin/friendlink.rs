use crate::entity::FriendLink;
use crate::service::friendlinks as link_service;
use crate::DbConn;
use rocket::http::{Status};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::super::{User, ViewData};

#[get("/friendlinks/list")]
pub fn list_page(_user: User) -> Result<Template, Status> {
    Ok(Template::render("admin/friendlink/friendlinks", json!({})))
}

#[get("/friendlinks/api/all")]
pub fn api_friendlinks_list(_user: User, conn: DbConn) -> JsonValue {
    json!(link_service::all_friendlinks(&conn, false))
}

#[get("/friendlinks/edit/<id>")]
pub fn edit_page(_user: User, conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match link_service::get_friendlink(&conn, id) {
        Ok(link) => {
            view_data.add("friendlink", link);
            Ok(Template::render("admin/friendlink/editfriendlink", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/friendlinks/edit", format = "json", data = "<link>")]
pub fn edit(_user: User, conn: DbConn, link: Json<FriendLink>) -> JsonValue {
    json!(link_service::update(&conn, link.0))
}

#[get("/friendlinks/add")]
pub fn add_page() -> Result<Template, Status> {
    Ok(Template::render("admin/friendlink/addfriendlink", json!({})))
}

#[post("/friendlinks/add", format = "json", data = "<link>")]
pub fn add(_user: User, conn: DbConn, link: Json<FriendLink>) -> JsonValue {
    json!(link_service::add(&conn, link.0))
}

#[delete("/friendlinks/delete/<id>")]
pub fn delete(_user: User, conn: DbConn, id: i32) -> JsonValue {
    json!(link_service::delete(&conn, id))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_page,
        edit_page, 
        edit,
        add_page,
        add,
        api_friendlinks_list,
        delete,
    ]
}

