pub mod admin;
pub mod blog;
pub mod post;
pub mod pagination;
pub mod comment;
pub mod tag;
pub mod category;

use diesel::prelude::*;
use crate::schema::dict::dsl as dict_dsl;

fn err_str<T: ToString>(err: T) -> String {
    err.to_string()
}

pub fn get_dict_value(key: String, conn: &SqliteConnection) -> Option<String> {
    dict_dsl::dict.find(key).select(dict_dsl::d_value).first::<String>(conn).ok()
}