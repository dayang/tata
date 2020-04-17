use crate::DbConn;
use rocket_contrib::templates::Template;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use crate::service::{
    post as post_service,
    tag as tag_service,
    category as category_service,
    comment as comment_service,
    get_dict_value,
};
use std::collections::HashMap;
use crate::consts::*;
use crate::dto::comment::CommentRequest;

// #[get("/")]
// pub fn index(conn: DbConn) -> Template{
//     let all_articles = BlogTask::get_article_briefs(&conn);
//     Template::render("index", &json!({
//         "title": "Yonghua's Blog".to_string(),
//         "articles": all_articles,
//     }))
// }


#[get("/?<page>")]
pub fn get_posts(page: Option<i32>, conn: DbConn) -> Result<Template, Status> {
    let mut bag = HashMap::new();
    bag.insert("title", get_dict_value(DICT_INDEX_TITLE.into(), &conn).unwrap_or_else(|| "Yong Hua' blog".into()));
    bag.insert("list_title", "所有文章".into());
    match post_service::get_posts_list(&conn, page.unwrap_or(1), None, None) {
        Ok(post_list_info) => {
            Ok(Template::render("index", &json!({
                "viewbag": bag,
                "post_list_info": post_list_info,
            })))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/post/<url>")]
pub fn get_post(url: String, conn: DbConn) -> Result<Template, Status> {
    match post_service::get_post_detail(&conn, url) {
        Ok(post_detail) => {
            Ok(Template::render("post", &json!(post_detail)))
        },
        Err(_) => Err(Status::NotFound)
    }
}

#[put("/post/<url>/like")]
pub fn like_post(url: String, conn: DbConn) -> JsonValue {
    json!({
        "success" : post_service::like_post(&conn, url)
    })
}

#[post("/post/<url>/comment", format = "json", data = "<comment>")]
pub fn comment_post(url: String, conn: DbConn, comment: Json<CommentRequest>) -> JsonValue {
    json!({})
}

#[get("/post/<url>/comments?<page>")]
pub fn get_post_comments(url: String, page: Option<i32>, conn: DbConn) -> JsonValue {
    match post_service::get_post_by_url(&conn, url) {
        Ok(post_find) => json!({
            "success": true,
            "data": comment_service::get_paged_comment(&conn, COMMENT_FOR_POST, page.unwrap_or(1), post_find.id)
        }),
        Err(_) => json!({
            "success": false
        })
    }
}

#[get("/tags")]
pub fn tag_list(conn: DbConn)  -> Result<Template, Status> {
    let mut bag: HashMap<&str, String> = HashMap::new();
    bag.insert("title", "所有标签".into());
    bag.insert("list_title", "所有标签".into());
    match tag_service::all_tags(&conn) {
        Ok(all_tags) => {
            Ok(Template::render("tags", &json!({
                "viewbag": bag,
                "tags": all_tags,
            })))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/tag/<tag>?<page>")]
pub fn get_posts_by_tag(tag: String, page: Option<i32>, conn: DbConn) -> Result<Template, Status> {
    let tag_find = tag_service::get_tag_by_name(&conn, tag).map_err(|_| Status::NotFound)?;
    let mut bag = HashMap::new();
    bag.insert("title", tag_find.display_text.to_string());
    bag.insert("list_title", tag_find.display_text.to_string());
    match post_service::get_posts_list(&conn, page.unwrap_or(1), None, Some(tag_find.id)) {
        Ok(post_list_info) => {
            Ok(Template::render("index", &json!({
                "viewbag": bag,
                "post_list_info": post_list_info,
            })))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/category/<category>?<page>")]
pub fn get_posts_by_category(category: String, page: Option<i32>, conn: DbConn) -> Result<Template, Status> {
    let category_find = category_service::get_category_by_name(&conn, category).map_err(|_| Status::NotFound)?;
    let mut bag = HashMap::new();
    bag.insert("title", category_find.display_text.to_string());
    bag.insert("list_title", category_find.display_text.to_string());
    match post_service::get_posts_list(&conn, page.unwrap_or(1), Some(category_find.id), None) {
        Ok(post_list_info) => {
            Ok(Template::render("index", &json!({
                "viewbag": bag,
                "post_list_info": post_list_info,
            })))
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