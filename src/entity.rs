use chrono::prelude::*;
use crate::schema::*;

#[derive(Queryable, Serialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub create_at: NaiveDateTime,
    pub published: bool,
    pub category_id: i32,
    pub visit_count: i32,
}

#[derive(Queryable, Serialize, Insertable)]
pub struct Category {
    pub id: i32,
    pub label: String,
}