pub mod book;
pub mod category;
pub mod comment;
pub mod dict;
pub mod friendlink;
pub mod logininfo;
pub mod page;
pub mod post;
pub mod tag;
pub mod user;
pub mod image;

// use crate::DbConn;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::State;
use rocket::response::Redirect;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use super::ViewData;
use super::{Auth, JsonErr, User};
use crate::DbConn;

#[get("/index")]
pub fn index(_user: User) -> Result<Template, Status> {
    Ok(Template::render("admin/index", json!({})))
}

#[get("/dashboard")]
pub fn dashboard(_user: User, conn: DbConn) -> Result<Template, Status> {
    let mut view_data = ViewData::default();
    match crate::service::get_dashboard_data(&conn) {
        Ok(data) => {
            view_data.add("dashboard", data);
            Ok(Template::render("admin/dashboard", view_data.to_json()))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/login")]
pub fn login_redirect(_user: User) -> Redirect {
    Redirect::to("/admin/index")
}

#[get("/login", rank = 2)]
pub fn login_page() -> Result<Template, Status> {
    Ok(Template::render("admin/login", json!({})))
}

#[post("/login", format = "json", data = "<login_form>")]
pub fn login(mut cookies: Cookies, login_form: Json<Auth>, auth: State<Auth>) -> JsonValue {
    let err_captcha = cookies
        .get_private("code")
        .and_then(|cookie| {
            Some(cookie.value().to_lowercase() != (&login_form.0.captcha).to_lowercase())
        })
        .unwrap_or(false);
    if err_captcha {
        return json!(JsonErr::Err("验证码不正确呦~".into()));
    }

    if login_form.0.username == auth.username && login_form.0.password == auth.password {
        let mut cookie = Cookie::new("auth", auth.username.clone());
        cookie.set_expires(time::now_utc() + time::Duration::days(1));
        cookie.set_max_age(time::Duration::days(1));
        // println!("{}", cookie) ;
        cookies.add_private(cookie);
        json!(JsonErr::Ok("success".into()))
    } else {
        json!(JsonErr::Err("login failed".into()))
    }
}

#[get("/logout")]
pub fn logout(_user: User, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("auth"));
    Redirect::to("/admin/login")
}

#[get("/logout", rank = 2)]
pub fn logout_invalid() -> JsonValue {
    json!(JsonErr::Err("user not login".into()))
}

pub fn routes() -> Vec<rocket::Route> {
    let mut routes = routes![
        index,
        dashboard,
        login,
        login_redirect,
        login_page,
        logout,
        logout_invalid,
    ];

    routes.append(&mut tag::routes());
    routes.append(&mut category::routes());
    routes.append(&mut friendlink::routes());
    routes.append(&mut comment::routes());
    routes.append(&mut post::routes());
    routes.append(&mut book::routes());
    routes.append(&mut page::routes());
    routes.append(&mut image::routes());

    routes
}
