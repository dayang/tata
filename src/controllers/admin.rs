use crate::DbConn;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::{Template};
use rocket::request::{Form, State};
use rocket::http::{Cookies, Cookie};
use crate::tasks::admin as AdminTask;
use crate::dto::ApiResponse;
use super::User;
use super::Auth;

macro_rules! admin_or_login {
    ($admin: ident, $t: expr) => {
        if $admin.is_admin() {
            Ok($t)
        } else {
            Err(Redirect::to("/admin/login"))
        }
    };
}

#[get("/login")]
fn login_page(admin: User) -> Result<Redirect, Option<NamedFile>> {
    if admin.is_admin() {
        Ok(Redirect::to("/admin/"))
    } else {
        Err(NamedFile::open("templates/admin/login.html").ok())
    }
}

#[post("/login", data = "<login_field>")]
fn login(mut cookies: Cookies, login_field: Form<Auth>, auth: State<Auth>) -> Redirect {
    if login_field.admin == auth.admin && login_field.password == auth.password {
        let mut cookie = Cookie::new("auth", auth.admin.clone());
        cookie.set_expires(time::now_utc() + time::Duration::days(1));
        cookie.set_max_age(time::Duration::minutes(30));
        println!("{}", cookie) ;
        cookies.add_private(cookie);
        Redirect::to("/admin/")
    } else {
        Redirect::to("/admin/login")
    }
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("auth"));
    Redirect::to("/admin/login")
}

#[get("/")]
fn index(admin: User) -> Result<Template, Redirect> {
    admin_or_login!(admin,
        Template::render("admin/index", &json!({
            "header" : "首页",
        }))
    )
}

// 管理文章页面
#[get("/manage-articles")]
fn manage_article_index(admin: User, conn: DbConn) -> Result<Template, Redirect> {
    admin_or_login!(admin,
        Template::render("admin/manage-articles", &json!({
            "header" : "文章管理",
            "articles" : AdminTask::get_admin_article_briefs(&conn)
        }))
    )
}

// 新建文章页面
#[get("/add-article")]
fn add_article_index(admin: User, conn: DbConn) -> Result<Template, Redirect> {
    admin_or_login!(admin,
        Template::render("admin/add-article", &json!({
            "header" : "添加文章",
            "categorys" : AdminTask::get_all_categorys(&conn)
        }))
    )
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
#[get("/put-article/<id>")]
fn put_article_index(admin: User, conn: DbConn, id: i32) -> Result<Template, Redirect> {
    admin_or_login!(admin,
        Template::render("admin/put-article", &json!({
            "header" : "更新文章",
            "categorys" : AdminTask::get_all_categorys(&conn)
        }))
    )
}

// 修改文章接口
#[put("/article", format = "json", data = "<article>")]
fn put_article(conn: DbConn, article: Json<crate::dto::admin::PostArticleDto>) -> JsonValue {
    let success = AdminTask::update_article(&conn, article.0);
    json!(ApiResponse {
        success: success,
        err_msg: match success {
            true => "成功".to_string(),
            false => "失败".to_string()
        }
    })
}

// 发布/取消发布 接口
#[post("/article/toggle-published/<id>")]
fn change_article_published(conn: DbConn, id: i32) ->JsonValue {
    let success = AdminTask::toggle_article_published(&conn, id);
    json!(ApiResponse {
        success: success,
        err_msg: match success {
            true => "成功".to_string(),
            false => "失败".to_string()
        }
    })
}

// 管理分类页面
#[get("/manage-categorys")]
fn manage_category_index(admin: User, conn: DbConn) -> Result<Template, Redirect> {
    admin_or_login!(admin,
        Template::render("admin/manage-categorys", &json!({
            "header" : "管理分类",
            "categorys" : AdminTask::get_all_categorys(&conn)
        }))
    )
}

// 增加分类接口
#[post("/category", format = "json", data = "<category>")]
fn add_category(conn: DbConn, category: Json<crate::dto::admin::CategoryDto>) ->JsonValue {
    let success = AdminTask::insert_category(&conn, category.0);
    json!(ApiResponse {
        success: success,
        err_msg: match success {
            true => "成功".to_string(),
            false => "失败".to_string()
        }
    })
}

// 删除分类接口
#[delete("/category/<id>")]
fn delete_category(id: i32) {

}

// 修改分类接口
#[put("/category", format = "json", data = "<category>")]
fn put_category(conn: DbConn, category: Json<crate::dto::admin::CategoryDto>) ->JsonValue {
    let success = AdminTask::update_category(&conn, category.0);
    json!(ApiResponse {
        success: success,
        err_msg: match success {
            true => "成功".to_string(),
            false => "失败".to_string()
        }
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login_page,
        login,
        logout,
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