use super::err_str;
use crate::entity::Category;
use crate::schema::category::dsl::*;
use diesel::prelude::*;

pub fn get_category_by_name(
    conn: &SqliteConnection,
    category_name: String,
) -> Result<Category, String> {
    category
        .filter(name.eq(category_name))
        .first::<Category>(conn)
        .map_err(err_str)
}

pub fn all_categorys(conn: &SqliteConnection) -> Result<Vec<Category>, String> {
    category.load::<Category>(conn).map_err(err_str)
}

pub fn get_category(conn: &SqliteConnection, category_id: i32) -> Result<Category, String> {
    category.find(category_id).first::<Category>(conn).map_err(err_str)
}


pub fn add(conn: &SqliteConnection, category_field: Category) -> Result<usize, String> {
    diesel::insert_into(category).values((
        name.eq(category_field.name),
        display_text.eq(category_field.display_text),
        remark.eq(category_field.remark),
    )).execute(conn).map_err(err_str)
}

pub fn update(conn: &SqliteConnection, category_field: Category) -> Result<usize, String> {
    diesel::update(&category_field).set(&category_field).execute(conn).map_err(err_str)
}

pub fn delete(conn: &SqliteConnection, delete_id: i32) -> Result<usize, String> {
    use crate::schema::post::dsl::{ post, category_id};
    use crate::entity::Post;
    if post.filter(category_id.eq(delete_id)).first::<Post>(conn).is_ok() {
        return Err("该分类被使用中，不能删除".into());
    }

    diesel::delete(category.filter(id.eq(delete_id))).execute(conn).map_err(err_str)
}
