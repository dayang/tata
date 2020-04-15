use crate::DbConn;
use rocket_contrib::templates::Template;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use crate::dto::comment::{Comment, CommentRequest};

use super::User;

// #[get("/")]
// pub fn index(conn: DbConn) -> Template{
//     let all_articles = BlogTask::get_article_briefs(&conn);
//     Template::render("index", &json!({
//         "title": "Yonghua's Blog".to_string(),
//         "articles": all_articles,
//     }))
// }

#[get("/?<page>")]
pub fn posts(page: i32, conn: DbConn, user: User) -> Result<Template, Status> {
    
}

#[get("/post/<url>")]
pub fn post(url: String, conn: DbConn, user: User)  { //-> Result<Template, Status>
    // match BlogTask::get_article_by_url(&conn, id, !user.is_admin()) {
    //     Some(data) => Ok(Template::render("article", &data)),
    //     None => Err(Status::NotFound)
    // }
}

#[put("/post/<url>/like")]
pub fn like_post(url: String, conn: DbConn) {

}

#[post("/post/<url>/comment", format = "json", data = "<comment>")]
pub fn comment_post(url: String, conn: DbConn, comment: Json<CommentRequest>) {

}

#[get("/post/<url>/comments?<page>")]
pub fn post_comments(url: String, page: i32, conn: DbConn) {

}

#[get("/tags")]
pub fn tag_list(conn: DbConn, user: User) {

}

#[get("/tag/<tag>?<page>")]
pub fn posts_by_tag(tag: String, page: i32, conn: DbConn, user: User) {

}

#[get("/category/<category>?<page>")]
pub fn posts_by_category(category: String, page: i32, conn: DbConn, user: User) {

}

#[get("/archive")]
pub fn archive() {

}

#[get("/archive/<year>/<month>")]
pub fn archive_month(year: String, month: String, conn: DbConn, user: User) {

}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        posts,
        post,
        like_post,
        comment_post,
        post_comments,
        tag_list,
        posts_by_tag,
        posts_by_category,
        archive,
        archive_month,
    ]
}