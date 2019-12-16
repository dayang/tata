use crate::DbConn;
use rocket_contrib::templates::Template;
use crate::tasks::blog as BlogTask;

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
pub fn article(id: i32, conn: DbConn) -> Template {
    Template::render("article", BlogTask::get_article_brief_by_id(&conn, id))
}

#[get("/article-body/<id>")]
pub fn get_article_body(id: i32, conn: DbConn) -> String {
    BlogTask::get_article_body_by_id(&conn, id)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        article,
        get_article_body
    ]
}