use super::err_str;
use crate::entity::Tag;
use crate::schema::tag::dsl::*;
use diesel::prelude::*;

pub fn get_tag_by_name(conn: &SqliteConnection, tag_name: String) -> Result<Tag, String> {
    tag.filter(name.eq(tag_name))
        .first::<Tag>(conn)
        .map_err(err_str)
}

pub fn get_tag(conn: &SqliteConnection, tag_id: i32) -> Result<Tag, String> {
    tag.find(tag_id).first::<Tag>(conn).map_err(err_str)
}

pub fn all_tags(conn: &SqliteConnection) -> Result<Vec<Tag>, String> {
    tag.load::<Tag>(conn).map_err(err_str)
}

pub fn add(conn: &SqliteConnection, tag_field: Tag) -> Result<usize, String> {
    diesel::insert_into(tag).values((
        name.eq(tag_field.name),
        display_text.eq(tag_field.display_text),
        remark.eq(tag_field.remark),
        weight.eq(tag_field.weight)
    )).execute(conn).map_err(err_str)
}

pub fn update(conn: &SqliteConnection, tag_field: Tag) -> Result<usize, String> {
    diesel::update(&tag_field).set(&tag_field).execute(conn).map_err(err_str)
}

pub fn delete(conn: &SqliteConnection, delete_id: i32) -> Result<usize, String> {
    use crate::schema::posttag::dsl::{ posttag, tag_id};
    use crate::entity::Posttag;
    if posttag.filter(tag_id.eq(delete_id)).first::<Posttag>(conn).is_ok() {
        return Err("该标签被使用中，不能删除".into());
    }

    diesel::delete(tag.filter(id.eq(delete_id))).execute(conn).map_err(err_str)
}