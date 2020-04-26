#[derive(Serialize, Deserialize)]
pub struct CommentListItem {
    pub user_name: String,
    pub avatar: String,
    pub content: String,
    pub comment_time: String,
    pub reply: String,
    pub reply_time: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentListItemAdmin {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub comment_time: String,
    pub unread: bool,
    pub show: bool,
    pub comment_type: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CommentRequest {
    pub url: String,
    pub user_name: String,
    pub email: String,
    pub content: String,
    pub captcha: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub id: i32,
    pub reply: String,
    pub show: bool,
}
