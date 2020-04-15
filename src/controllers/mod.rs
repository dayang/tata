// pub mod admin;
pub mod post;

use rocket_contrib::templates::Template;
use rocket::request::{self, Request, FromRequest, State};
use rocket::outcome::IntoOutcome;
use rocket::response::{Redirect};

#[derive(FromForm)]
pub struct Auth{
    #[form(field = "username")]
    pub admin: String,
    pub password: String,
}

pub struct User(bool);

impl User {
    pub fn is_admin(&self) -> bool{
        self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let auth = request.guard::<State<Auth>>()?;
        request.cookies()
            .get_private("auth")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id: String| Some(User(auth.admin == id)))
            .or(Some(User(false)))
            .or_forward(())
    }
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", &json!({}))
}

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to("/static/favicon.ico")
}