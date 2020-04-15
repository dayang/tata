#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub user_name: String,
    pub content: String,
    //#[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub comment_time: String,
    pub reply: Option<String>,
    //#[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub reply_time: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentRequest {
    pub post_url: String,
    pub user_name: String,
    pub email: String,
    pub content: String,
}