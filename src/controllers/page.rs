use crate::service::{book as book_service, comment as comment_service, page as page_service};
use crate::DbConn;
use rocket::http::{Cookie, Cookies, Status};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::{JsonErr, ViewData};
use crate::consts::*;
use crate::dto::comment::CommentRequest;

#[get("/<url>")]
pub fn get_page(mut cookies: Cookies, url: String, conn: DbConn) -> Result<Template, Status> {
    let new_hit = cookies
        .get_private("hit")
        .and_then(|cookie| Some(cookie.value() != &url))
        .unwrap_or(true);

    let mut view_data = ViewData::default();
    view_data.load_posts_page_meta_data(&conn);
    match page_service::get_page_by_url(&conn, url.clone(), new_hit) {
        Ok(page) => {
            let book_id = page.book_id;

            view_data.add_viewbag("title", page.title.clone());
            view_data.add("page", page);

            if new_hit {
                let mut cookie = Cookie::new("hit", url);
                cookie.set_expires(time::now_utc() + time::Duration::days(1));
                cookie.set_max_age(time::Duration::days(1));
                cookies.add_private(cookie);
            }

            match book_service::get_book(&conn, book_id) {
                Ok(book) => view_data.add_viewbag("book_title", book.display_text),
                Err(_) => (),
            };

            match page_service::get_book_catalog(&conn, book_id) {
                Ok(catalog) => {
                    view_data.add("catalog", catalog);
                    Ok(Template::render("page", view_data.to_json()))
                }
                Err(_) => Err(Status::InternalServerError),
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[put("/<url>/like")]
pub fn like_page(mut cookies: Cookies, url: String, conn: DbConn) -> JsonValue {
    let have_liked = cookies
        .get_private("like")
        .and_then(|cookie| Some(cookie.value() == &url))
        .unwrap_or(false);
    if have_liked {
        json!(JsonErr::Err("你已经点过赞啦".into()))
    } else {
        let mut cookie = Cookie::new("like", url.clone());
        cookie.set_expires(time::now_utc() + time::Duration::days(1));
        cookie.set_max_age(time::Duration::days(1));
        cookies.add_private(cookie);

        json!(page_service::like_page(&conn, url).map_err(|_| "发生错误了呢"))
    }
}

#[post("/<url>/comment", format = "json", data = "<comment>")]
pub fn comment_page(
    mut cookies: Cookies,
    url: String,
    conn: DbConn,
    comment: Json<CommentRequest>,
) -> JsonValue {
    let err_captcha = cookies
        .get_private("code")
        .and_then(|cookie| {
            Some(cookie.value().to_lowercase() != (&comment.0.captcha).to_lowercase())
        })
        .unwrap_or(false);
    if err_captcha {
        return json!(JsonErr::Err("验证码不正确呦~".into()));
    }

    match page_service::get_page_by_url(&conn, url, false) {
        Ok(page_find) => json!(comment_service::new_comment(
            &conn,
            comment.0,
            page_find.id,
            COMMENT_FOR_PAGE
        )),
        Err(_) => json!(JsonErr::Err("page not found".into())),
    }
}

#[get("/<url>/comments?<page>&<_limit>")]
pub fn get_page_comments(
    url: String,
    page: Option<i32>,
    _limit: Option<i32>,
    conn: DbConn,
) -> JsonValue {
    match page_service::get_page_by_url(&conn, url, false) {
        Ok(page_find) => json!(comment_service::get_paged_comment(
            &conn,
            COMMENT_FOR_PAGE,
            page.unwrap_or(1),
            page_find.id
        )),
        Err(_) => json!(JsonErr::Err("post not found".to_string())),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_page, like_page, comment_page, get_page_comments,]
}
