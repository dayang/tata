pub mod admin;
pub mod book;
pub mod captcha;
pub mod friendlink;
pub mod page;
pub mod post;

use crate::consts::*;
use crate::service::get_dict_value;
use crate::service::{category as category_service, friendlinks as friendlink_service};
use crate::DbConn;
use diesel::prelude::*;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request, State};
use rocket::response::Redirect;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use serde::Serialize;
use serde_json::{to_value, Value as SerdeJsonValue};
use std::collections::HashMap;

type JsonErr = Result<String, String>;

#[derive(Clone)]
pub struct ViewData {
    view_bag: HashMap<String, SerdeJsonValue>,
    data: HashMap<String, SerdeJsonValue>,
}

impl Default for ViewData {
    fn default() -> Self {
        ViewData {
            view_bag: HashMap::new(),
            data: HashMap::new(),
        }
    }
}

impl ViewData {
    pub fn add_viewbag<K: ToString, V: Serialize>(&mut self, key: K, value: V) {
        self.view_bag
            .insert(key.to_string(), Self::json_value(value));
    }

    fn json_value<V: Serialize>(value: V) -> SerdeJsonValue {
        to_value(value).unwrap()
    }

    pub fn add<K: ToString, V: Serialize>(&mut self, key: K, value: V) {
        if key.to_string() == "viewwbag" {
            panic!("cannot add 'viewbag', please user add_viewbag function");
        }
        self.data.insert(key.to_string(), Self::json_value(value));
    }

    pub fn to_json(self) -> JsonValue {
        let view_bag = self.view_bag;
        let mut data = self.data;
        data.insert("viewbag".into(), Self::json_value(view_bag));
        json!(data)
    }

    pub fn load_posts_page_meta_data(&mut self, conn: &SqliteConnection) {
        match category_service::all_categorys(conn) {
            Ok(categorys) => {
                self.view_bag
                    .insert("categorys".into(), Self::json_value(categorys));
            }
            Err(_) => (),
        };

        get_dict_value(DICT_INDEX_QUOTE.into(), conn)
            .into_iter()
            .for_each(|v| {
                self.view_bag
                    .insert("index_quote".into(), Self::json_value(v));
            });
    }
}

#[derive(Serialize, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
    pub captcha: String,
}

pub struct User {
    pub user_id: i32,
    pub user_name: String,
}

// impl User {
//     pub fn is_logined(&self) -> bool {
//         self.0
//     }
// }

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let auth = request.guard::<State<Auth>>()?;
        request
            .cookies()
            .get_private("auth")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|user_name: String| match user_name == auth.username {
                true => Some(User {
                    user_id: 0,
                    user_name: user_name,
                }),
                false => None,
            })
            .or_forward(())
    }
}

#[get("/about")]
pub fn about(conn: DbConn) -> Template {
    let mut view_data = ViewData::default();
    view_data.load_posts_page_meta_data(&conn);
    view_data.add("about_page", get_dict_value(DICT_ABOUT_PAGE.into(), &conn));
    view_data.add("friendlink_apply_enabled", get_dict_value(DICT_FRIENDLINK_APPPLY_ENABLE, &conn).map(|b| b.parse::<bool>().unwrap_or(false)).unwrap_or(false));

    match friendlink_service::all_friendlinks(&conn, true) {
        Ok(links) => view_data.add("friend_links", links),
        Err(_) => (),
    };

    Template::render("about", view_data.to_json())
}

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to("/static/favicon.ico")
}
