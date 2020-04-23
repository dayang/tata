use crate::DbConn;
// use rocket_contrib::templates::Template;
use rocket_contrib::json::{Json, JsonValue};
// use rocket::http::{Status, Cookies, Cookie};
use crate::service::friendlinks as friendlink_service;

// use crate::consts::*;
use crate::dto::friendlink::ApplyFriendLink;
// use super::{ViewData, JsonErr};

#[post("/apply", format = "json", data = "<apply>")]
pub fn apply_for(conn: DbConn, apply: Json<ApplyFriendLink>) -> JsonValue {
    json!(friendlink_service::apply_for(&conn, apply.0))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![apply_for]
}
