pub mod admin;
pub mod book;
pub mod category;
pub mod comment;
pub mod friendlinks;
pub mod page;
pub mod pagination;
pub mod post;
pub mod tag;

use crate::schema::dict::dsl as dict_dsl;
use diesel::prelude::*;

fn err_str<T: ToString>(err: T) -> String {
    err.to_string()
}

pub fn get_dict_value(key: String, conn: &SqliteConnection) -> Option<String> {
    dict_dsl::dict
        .find(key)
        .select(dict_dsl::d_value)
        .first::<String>(conn)
        .ok()
}
