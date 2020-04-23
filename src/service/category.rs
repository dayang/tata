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
