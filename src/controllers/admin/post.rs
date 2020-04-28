use crate::dto::post::*;
use crate::service::post as post_service;
use crate::DbConn;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::super::{User, ViewData};

#[get("/posts/list")]
pub fn list_page(_user: User) -> Result<Template, Status> {
    Ok(Template::render("admin/post/posts", json!({})))
}

#[get("/posts/api/list?<page>&<limit>&<tag>&<category>&<year>&<month>&<published>")]
pub fn api_posts_list(
    _user: User,
    conn: DbConn,
    page: Option<i32>,
    limit: Option<i32>,
    tag: Option<i32>,
    category: Option<i32>,
    year: Option<i32>,
    month: Option<i32>,
    published: Option<bool>,
) -> JsonValue {
    json!(post_service::get_posts_list(
        &conn,
        page.unwrap_or(1),
        limit,
        category,
        tag,
        if year.is_some() && month.is_some() {
            Some((year.unwrap(), month.unwrap()))
        } else {
            None
        },
        published
    ))
}

#[get("/posts/edit/<id>")]
pub fn edit_page(_user: User, conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match post_service::get_post(&conn, id) {
        Ok(post) => {
            view_data.add("post", post);
            Ok(Template::render("admin/post/editpost", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/posts/edit", format = "json", data = "<update_post>")]
pub fn edit(_user: User, conn: DbConn, update_post: Json<CreateOrUpdatePost>) -> JsonValue {
    json!(post_service::update_post(&conn, update_post.0))
}

#[delete("/posts/delete/<id>")]
pub fn delete(_user: User, conn: DbConn, id: i32) -> JsonValue {
    json!(post_service::delete(&conn, id))
}

#[get("/posts/add")]
pub fn add_page() -> Result<Template, Status> {
    Ok(Template::render("admin/post/addpost", json!({})))
}

#[post("/posts/add", format = "json", data = "<create_post>")]
pub fn add(_user: User, conn: DbConn, create_post: Json<CreateOrUpdatePost>) -> JsonValue {
    json!(post_service::create_post(&conn, create_post.0))
}

#[get("/posts/post/<id>/tags")]
pub fn get_tags(_user:User, conn: DbConn, id: i32) -> JsonValue {
    json!(post_service::get_post_tags(&conn, id))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_page,
        edit_page,
        edit,
        api_posts_list,
        delete,
        add_page,
        add,
        get_tags,
    ]
}
