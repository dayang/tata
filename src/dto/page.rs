#[derive(Serialize, Deserialize)]
pub struct UpdatePage {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub content: String,
    pub allow_comment: bool,
    pub published: bool,
}
