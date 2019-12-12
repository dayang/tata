pub mod admin;

use crate::models::Article;

#[derive(Serialize)]
pub struct PageArticles {
    pub title: String,
    pub articles: Vec<Article>,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub err_msg: String,
}