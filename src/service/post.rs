use crate::dto::post::Post;
use crate::DbConn;
use diesel::prelude::*;
use crate::schema::post::dsl::*;
use crate::schema::category::dsl::*;
use crate::sqltypes::*;
use crate::entity::Post as PostEntity;


pub fn get_posts(conn: DbConn, page_num: i32, page_index: i32, by_category_id: Option<i32>, by_tag_id: Option<i32>) -> Result<Vec<Post>, String> {
    if page_num < 1 {
        return Err("page_num must greater than 0".into());
    }

    if page_index < 1 {
        return Err("page index must great than 0".into());
    }

    let mut filtered = post.filter(published.eq(true)).into_boxed();
    if let Some(by_id) = by_category_id {
        filtered = filtered.filter(category_id.eq(by_id));
    }

    let result = filtered.offset(((page_index - 1) * page_num) as i64).limit(page_num as i64).load::<QueryModel>(&conn.0);
    match result {
        Ok(query_results) => {
            Ok(vec![])
        },
        Err(_) => {
            return Err("sql execute error".into());
        }
    }

    //post.filter(published.eq(true)).offset((page_index - 1) * page_num).limit(page_num).
}