pub mod book;
pub mod comment;
pub mod dict;
pub mod friendlink;
pub mod page;
pub mod pagination;
pub mod post;

#[derive(Serialize)]
pub struct DashBoardData {
    pub post_count: i64,
    pub book_count: i64,
    pub page_count: i64,
    pub comment_count: i64,
    pub unread_comments: i64,
    pub friendlink_count: i64,
}
