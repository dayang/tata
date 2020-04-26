use crate::consts::*;
use crate::dto::comment::*;
use crate::service::comment as comment_service;
use crate::DbConn;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::super::{User, ViewData};

#[get("/comments/list")]
pub fn list_page(_user: User) -> Result<Template, Status> {
    Ok(Template::render("admin/comment/comments", json!({})))
}

#[get("/comments/api/list?<page>&<limit>&<for_type>&<for_id>&<only_read>")]
pub fn api_comments_list(
    _user: User,
    conn: DbConn,
    page: Option<i32>,
    limit: Option<i32>,
    for_type: Option<i32>,
    for_id: Option<i32>,
    only_read: Option<bool>,
) -> JsonValue {
    json!(comment_service::get_paged_comment_admin(
        &conn,
        for_type,
        for_id,
        page.unwrap_or(1),
        limit.unwrap_or(10),
        only_read.unwrap_or(false)
    ))
}

#[get("/comments/edit/<id>")]
pub fn edit_page(_user: User, conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match comment_service::get_comment(&conn, id, true) {
        Ok(comment) => {
            if comment.0.comment_type == COMMENT_FOR_POST {
                view_data.add_viewbag("url", format!("/posts/post/{}", &comment.1));
            } else if comment.0.comment_type == COMMENT_FOR_PAGE {
                todo!()
                //view_data.add_viewbag("url", format!("/posts/post/{}", &comment.1));
            }

            view_data.add("comment", comment.0);
            Ok(Template::render(
                "admin/comment/editcomment",
                view_data.to_json(),
            ))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/comments/edit", format = "json", data = "<update_comment>")]
pub fn edit(_user: User, conn: DbConn, update_comment: Json<UpdateCommentRequest>) -> JsonValue {
    json!(comment_service::update(&conn, update_comment.0))
}

#[delete("/comments/delete/<id>")]
pub fn delete(_user: User, conn: DbConn, id: i32) -> JsonValue {
    json!(comment_service::delete(&conn, id))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_page, edit_page, edit, api_comments_list, delete,]
}
