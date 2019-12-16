#[derive(Serialize)]
pub struct ArticleBrief{
    pub id: i32,
    pub title: String,
    pub create_at: String,
    pub category: String,
    pub visit_count: i32,
}
