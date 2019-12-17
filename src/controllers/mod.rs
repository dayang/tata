pub mod admin;
pub mod blog;

use rocket::request::{self, Request, FromRequest, State};
use rocket::outcome::IntoOutcome;
use rocket::response::{Redirect};
use rocket::http::Status;

pub struct Auth{
    pub admin: String,
    pub password: String,
}

struct AdminUser(bool);
struct AdminUserOrLogin;

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, ()> {
        let auth = request.guard::<State<Auth>>()?;
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id: String| Some(AdminUser(auth.admin == id)))
            .or_forward(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUserOrLogin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUserOrLogin, ()> {
        let auth = request.guard::<State<Auth>>()?;
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id: String| match id == auth.admin {
                true => Some(AdminUserOrLogin),
                false => None
            }).or_forward(())
    }
}