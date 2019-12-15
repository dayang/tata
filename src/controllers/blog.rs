use crate::DbConn;
use rocket_contrib::templates::Template;
use crate::dto::PageArticles;
use crate::tasks::admin as AdminTask;

#[get("/")]
pub fn index(conn: DbConn) -> Template{
    let all_articles = AdminTask::all(&conn);
    Template::render("index", &PageArticles {
        title: "Yonghua's blog".to_string(),
        articles: all_articles,
    })
}

// #[get("/<category>")]
// pub fn category(category: String) -> String {
//     category
// }

#[get("/<id>")]
pub fn article(id: i32, conn: DbConn) -> Template {
    Template::render("article", AdminTask::get_article_by_id(&conn, id))
}

#[get("/v1/<id>")]
pub fn get_article(id: i32, conn: DbConn) -> String {
    AdminTask::get_article_by_id(&conn, id).body
}


pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        article,
        get_article
    ]
}