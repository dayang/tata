use crate::dto::dict::*;
use crate::service::dict as dict_service;
use crate::DbConn;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::super::{User, ViewData};

#[get("/dicts")]
pub fn dicts_page(_user: User, conn: DbConn) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match dict_service::get_dicts(&conn) {
        Ok(dicts) => {
            view_data.add("dicts", dicts);
            Ok(Template::render("admin/dict/dicts", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/dicts/edit", format = "json", data = "<update_dicts>")]
pub fn edit(_user: User, conn: DbConn, update_dicts: Json<Dicts>) -> JsonValue {
    json!(dict_service::update_dicts(&conn, update_dicts.0))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![dicts_page, edit]
}
