use crate::DbConn;
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::{Template};
use crate::tasks::admin as AdminTask;
use crate::dto::ApiResponse;

#[get("/login")]
fn login() -> Option<NamedFile> {
    NamedFile::open("templates/admin/login.html").ok()
}

#[get("/")]
fn index() -> Template {
    Template::render("admin/index", &json!({
        "header" : "首页",
    }))
}

// 管理文章页面
#[get("/manage-articles")]
fn manage_article_index(conn: DbConn) -> Template {
    Template::render("admin/manage-articles", &json!({
        "header" : "文章管理",
        "articles" : AdminTask::get_admin_article_briefs(&conn)
    }))
}

// 新建文章页面
#[get("/add-article")]
fn add_article_index() -> &'static str {
    "/add-article"
}

// 新建文章接口
#[post("/article", format = "json", data = "<article>")]
fn add_article(conn: DbConn, article: Json<crate::dto::admin::PostArticleDto>) -> JsonValue {
    let success = AdminTask::insert_article(&conn, article.0);
    json!(ApiResponse {
        success: success,
        err_msg: match success {
            true => "成功".to_string(),
            false => "失败".to_string()
        }
    })
}

// 删除文章接口
#[delete("/article/<id>")]
fn delete_article(id: i32) {

}

// 修改文章页面
#[get("/put-article")]
fn put_article_index() -> &'static str {
    "/put-article"
}

// 修改文章接口
#[put("/article")]
fn put_article() {

}

// 发布/取消发布 接口
#[post("/article/toggle-published")]
fn change_article_published() {

}

// 管理分类页面
#[get("/manage-categorys")]
fn manage_category_index() -> &'static str {
    "/manage-category"
}

// 增加分类接口
#[post("/category")]
fn add_category() {

}

// 删除分类接口
#[delete("/category/<id>")]
fn delete_category(id: i32) {

}

// 修改分类接口
#[put("/category")]
fn put_category() {

}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login,
        index,
        manage_article_index,
        add_article_index,
        add_article,
        delete_article,
        put_article,
        put_article_index,
        change_article_published,
        manage_category_index, 
        add_category,
        delete_category,
        put_category
    ]
}