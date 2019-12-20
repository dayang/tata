use crate::DbConn;
use rocket_contrib::templates::Template;
use rocket::http::Status;
use crate::tasks::blog as BlogTask;
use super::User;

#[get("/")]
pub fn index(conn: DbConn) -> Template{
    let all_articles = BlogTask::get_article_briefs(&conn);
    Template::render("index", &json!({
        "title": "Yonghua's blog".to_string(),
        "articles": all_articles,
    }))
}

// #[get("/<category>")]
// pub fn category(category: String) -> String {
//     category
// }

#[get("/<id>")]
pub fn article(id: i32, conn: DbConn, user: User) -> Result<Template, Status> {
    match BlogTask::get_article_brief_by_id(&conn, id, !user.is_admin()) {
        Some(data) => Ok(Template::render("article", &data)),
        None => Err(Status::NotFound)
    }
}

#[get("/article-body/<id>")]
pub fn get_article_body(id: i32, conn: DbConn, user: User) -> Option<String> {
    BlogTask::get_article_body_by_id(&conn, id, !user.is_admin())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        //index,
        article,
        get_article_body
    ]
}