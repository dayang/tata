use crate::DbConn;
use rocket_contrib::templates::Template;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::{Status, Cookies, Cookie};
use crate::service::{
    post as post_service,
    tag as tag_service,
    category as category_service,
    comment as comment_service,
    get_dict_value,
};

use crate::consts::*;
use crate::dto::comment::CommentRequest;
use super::{ViewData, JsonErr};

#[get("/")]
pub fn index(conn: DbConn) -> Result<Template, Status>{
    get_posts(None, conn)
}


#[get("/?<page>")]
pub fn get_posts(page: Option<i32>, conn: DbConn) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    view_data.add_viewbag("title", get_dict_value(DICT_INDEX_TITLE.into(), &conn).unwrap_or_else(|| "Yong Hua' blog".into()));
    view_data.add_viewbag("list_title", "所有文章");
    view_data.load_posts_page_meta_data(&conn);
    match post_service::get_posts_list(&conn, page.unwrap_or(1), None, None) {
        Ok(post_list_info) => {
            view_data.add("post_list_info", post_list_info);
            //println!("{}", view_data.clone().to_json().to_string());
            Ok(Template::render("index", view_data.to_json()))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/post/<url>")]
pub fn get_post(mut cookies: Cookies, url: String, conn: DbConn) -> Result<Template, Status> {
    let new_hit = cookies.get_private("hit").and_then(|cookie| Some(cookie.value() != &url)).unwrap_or(true);

    let mut view_data = ViewData::default();
    view_data.load_posts_page_meta_data(&conn);
    match post_service::get_post_detail(&conn, url.clone(), new_hit) {
        Ok(post_detail) => {
            view_data.add_viewbag("title", post_detail.title.clone());
            view_data.add("post", post_detail);

            if new_hit {
                let mut cookie = Cookie::new("hit", url);
                cookie.set_expires(time::now_utc() + time::Duration::days(1));
                cookie.set_max_age(time::Duration::days(1));
                cookies.add_private(cookie);
            }

            Ok(Template::render("post", view_data.to_json()))
        },
        Err(_) => Err(Status::NotFound)
    }
}

#[put("/post/<url>/like")]
pub fn like_post(mut cookies: Cookies, url: String, conn: DbConn) -> JsonValue {
    let have_liked = cookies.get_private("like").and_then(|cookie| Some(cookie.value() == &url)).unwrap_or(false);
    if have_liked {
        json!(JsonErr::Err("你已经点过赞啦".into()))
    } else {
        let mut cookie = Cookie::new("like", url.clone());
        cookie.set_expires(time::now_utc() + time::Duration::days(1));
        cookie.set_max_age(time::Duration::days(1));
        cookies.add_private(cookie);

        json!(post_service::like_post(&conn, url).map_err(|_| "发生错误了呢"))
    }
}

#[post("/post/<url>/comment", format = "json", data = "<comment>")]
pub fn comment_post(mut cookies: Cookies, url: String, conn: DbConn, comment: Json<CommentRequest>) -> JsonValue {
    let err_captcha = cookies.get_private("code").and_then(|cookie| Some(cookie.value().to_lowercase() != (&comment.0.captcha).to_lowercase())).unwrap_or(false);
    if err_captcha {
        return json!(JsonErr::Err("验证码不正确呦~".into()));
    }

    match post_service::get_post_by_url(&conn, url) {
        Ok(post_find) => json!(comment_service::new_comment(&conn, comment.0, post_find.id, COMMENT_FOR_POST)),
        Err(_) => json!(JsonErr::Err("post not found".into()))
    }
}

#[get("/post/<url>/comments?<page>")]
pub fn get_post_comments(url: String, page: Option<i32>, conn: DbConn) -> JsonValue {
    match post_service::get_post_by_url(&conn, url) {
        Ok(post_find) => json!(comment_service::get_paged_comment(&conn, COMMENT_FOR_POST, page.unwrap_or(1), post_find.id)),
        Err(_) => json!(JsonErr::Err("post not found".to_string()))
    }
}

#[get("/tags")]
pub fn tag_list(conn: DbConn)  -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    view_data.add_viewbag("title", "所有标签");
    view_data.add_viewbag("list_title", "所有标签");
    view_data.load_posts_page_meta_data(&conn);
    
    match tag_service::all_tags(&conn) {
        Ok(all_tags) => {
            view_data.add("tags", all_tags);
            Ok(Template::render("tags", view_data.to_json()))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/tag/<tag>?<page>")]
pub fn get_posts_by_tag(tag: String, page: Option<i32>, conn: DbConn) -> Result<Template, Status> {
    let tag_find = tag_service::get_tag_by_name(&conn, tag).map_err(|_| Status::NotFound)?;
    let mut view_data = ViewData::default();
    view_data.add_viewbag("title", tag_find.display_text.to_string());
    view_data.add_viewbag("list_title", tag_find.display_text.to_string());
    view_data.load_posts_page_meta_data(&conn);
    
    match post_service::get_posts_list(&conn, page.unwrap_or(1), None, Some(tag_find.id)) {
        Ok(post_list_info) => {
            view_data.add("post_list_info", post_list_info);
            Ok(Template::render("index", view_data.to_json()))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/category/<category>?<page>")]
pub fn get_posts_by_category(category: String, page: Option<i32>, conn: DbConn) -> Result<Template, Status> {
    let category_find = category_service::get_category_by_name(&conn, category).map_err(|_| Status::NotFound)?;
    let mut view_data = ViewData::default();
    view_data.add_viewbag("title", category_find.display_text.to_string());
    view_data.add_viewbag("list_title", category_find.display_text.to_string());
    view_data.load_posts_page_meta_data(&conn);
    
    match post_service::get_posts_list(&conn, page.unwrap_or(1), Some(category_find.id), None) {
        Ok(post_list_info) => {
            view_data.add("post_list_info", post_list_info);
            Ok(Template::render("index", view_data.to_json()))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/archive")]
pub fn archive() {

}

#[get("/archive/<year>/<month>")]
pub fn archive_month(year: String, month: String, conn: DbConn) {
    
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_posts,
        get_post,
        like_post,
        comment_post,
        get_post_comments,
        tag_list,
        get_posts_by_tag,
        get_posts_by_category,
        archive,
        archive_month,
    ]
}