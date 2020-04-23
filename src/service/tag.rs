use super::err_str;
use crate::entity::Tag;
use crate::schema::tag::dsl::*;
use diesel::prelude::*;

pub fn get_tag_by_name(conn: &SqliteConnection, tag_name: String) -> Result<Tag, String> {
    tag.filter(name.eq(tag_name))
        .first::<Tag>(conn)
        .map_err(err_str)
}

pub fn all_tags(conn: &SqliteConnection) -> Result<Vec<Tag>, String> {
    tag.load::<Tag>(conn).map_err(err_str)
}
