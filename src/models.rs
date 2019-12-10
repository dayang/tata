use chrono::prelude::*;

#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub create_at: DateTime<Local>,
    pub published: bool,
    pub category: i32,
    pub visit_count: i32,
}