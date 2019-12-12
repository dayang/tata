#[derive(Serialize, Deserialize)]
pub struct GetArticleBriefDto{
    pub id: i32,
    pub title: String,
    pub create_at: String,
    pub category: String,
    pub visit_count: i32,
    pub published: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PostArticleDto {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub category_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: i32,
    pub label: String,
}