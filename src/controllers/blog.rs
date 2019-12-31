use crate::DbConn;
use rocket_contrib::templates::Template;
use rocket::http::Status;
use crate::tasks::blog as BlogTask;
use super::User;

#[get("/")]
pub fn index(conn: DbConn) -> Template{
    let all_articles = BlogTask::get_article_briefs(&conn);
    Template::render("index", &json!({
        "title": "Yonghua's Blog".to_string(),
        "articles": all_articles,
    }))
}

// #[get("/<category>")]
// pub fn category(category: String) -> String {
//     category
// }

#[get("/<id>")]
pub fn article(id: i32, conn: DbConn, user: User) -> Result<Template, Status> {
    match BlogTask::get_article_by_id(&conn, id, !user.is_admin()) {
        Some(data) => Ok(Template::render("article", &data)),
        None => Err(Status::NotFound)
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        //index,
        article,
    ]
}