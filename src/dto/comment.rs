#[derive(Serialize, Deserialize)]
pub struct CommentListInfo {
    pub total_num: i64,
    pub total_pages: i64,
    pub per_page: i32,
    pub curr_page: i32,
    pub page_items: Vec<CommentListItem>,
}

impl Default for CommentListInfo {
    fn default() -> Self {
        CommentListInfo {
            total_num: 0,
            total_pages: 0,
            per_page: 0,
            curr_page: 0,
            page_items: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommentListItem {
    pub user_name: String,
    pub avatar: String,
    pub raw_content: String,
    pub html_content: String,
    pub comment_time: String,
    pub reply: Option<String>,
    pub reply_time: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentRequest {
    pub url: String,
    pub user_name: String,
    pub email: String,
    pub raw_content: String,
    pub html_content: String,
    pub captcha: String,
}
