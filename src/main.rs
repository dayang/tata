#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate mini_captcha;
extern crate time;

use diesel::SqliteConnection;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod catchers;
mod consts;
mod controllers;
mod dto;
mod entity;
mod helpers;
mod schema;
mod service;
mod sqltypes;
mod util;

embed_migrations!();

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(AdHoc::on_attach("Get Auth", |rocket| {
            let auth_config = rocket
                .config()
                .get_table("auth")
                .expect("missing auth config");

            let admin = auth_config
                .get("admin")
                .expect("missing auth:admin config")
                .as_str()
                .expect("admin should be string")
                .to_string();
            let password = auth_config
                .get("password")
                .expect("missing auth:password config")
                .as_str()
                .expect("password should be string")
                .to_string();

            Ok(rocket.manage(controllers::Auth { admin, password }))
        }))
        .mount(
            "/",
            routes![
                controllers::post::index,
                controllers::about,
                controllers::favicon
            ],
        )
        // .mount("/category", routes![controllers::category])
        .mount("/posts", controllers::post::routes())
        //.mount("/admin", controllers::admin::routes())
        .mount("/static", StaticFiles::from("static"))
        .mount("/captcha", routes![controllers::captcha::get_captcha])
        .mount("/friendlink", controllers::friendlink::routes())
        .register(catchers![catchers::not_found])
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("markdown", Box::new(crate::helpers::markdown_helper));
            engines
                .handlebars
                .register_helper("not_empty_str", Box::new(crate::helpers::NotEmptyStrHelper));
        }))
}

fn main() {
    rocket().launch();
}
