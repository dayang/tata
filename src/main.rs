#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate chrono;

use rocket_contrib::templates::{Template};

use rocket::Rocket;

mod schemas;
mod models;
mod controllers;
mod catchers;

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![controllers::index])
        .mount("/blog", routes![controllers::category])
        .register(catchers![catchers::not_found])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
