use crate::entity::Category;
use diesel::prelude::*;
use crate::schema::category::dsl::*;
use super::err_str;

pub fn get_category_by_name(conn: &SqliteConnection, category_name: String) -> Result<Category, String> {
    category.filter(name.eq(category_name)).first::<Category>(conn).map_err(err_str)
}