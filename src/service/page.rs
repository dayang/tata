use super::err_str;
use crate::dto::book::*;
use crate::entity::*;
use crate::schema::page::{dsl as page_dsl, dsl::*};
use diesel::prelude::*;
use std::collections::HashMap;

pub fn get_book_catalog(conn: &SqliteConnection, the_book_id: i32) -> Result<Vec<CatalogItem>, String> {
    let page_items = page
        .filter(page_dsl::book_id.eq(the_book_id))
        .select((id, parent_id, display_order, url))
        .load::<(i32, i32, i32, String)>(conn)
        .map_err(err_str)?;

    //let mut map = HashMap::new();
    let mut catalog_items = vec![];
    // for item in page_items {
        
    // }

    Err("error".into())
}
