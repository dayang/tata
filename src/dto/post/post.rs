use crate::entity::{Tag, Category};

#[derive(Serialize)]
pub struct PostListInfo {
    pub total_num: i32,
    pub total_pages: i32,
    pub curr_page: i32,
    pub page_items: Vec<PostListItem>,
}

#[derive(Serialize)]
pub struct PostListItem {
    pub title: String,
    pub url: String,
    pub summary: String,
    pub thumbnail: String,
    pub reads: i32,
    pub create_time: String,
    pub tags: Vec<Tag>,
}

use super::super::comment::Comment;
#[derive(Serialize)]
pub struct PostDetail {
    pub title: String,
    pub url: String,
    // pub raw_content: String,
    pub html_content: String,
    pub summary: String,
    pub thumbnail: String,
    pub reads: i32,
    pub likes: i32,
    pub allow_comment: bool,
    pub comments: Vec<Comment>,
    pub comments_count: i32,
    //#[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub create_time: String,
    //#[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub edit_time: String,
    pub tags: Vec<Tag>,
    pub category: Category,
}