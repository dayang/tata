use crate::entity::{Category, Tag};

#[derive(Serialize)]
pub struct PostListItem {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub summary: String,
    pub thumbnail: String,
    pub reads: i32,
    pub likes: i32,
    pub create_time: String,
    pub edit_time: String,
    pub category: Category,
    pub tags: Vec<Tag>,
    pub published: bool,
}

//use super::super::comment::*;
#[derive(Serialize)]
pub struct PostDetail {
    pub title: String,
    pub url: String,
    pub content: String,
    pub summary: String,
    pub thumbnail: String,
    pub reads: i32,
    pub likes: i32,
    pub allow_comment: bool,
    // pub comment_list_info: CommentListInfo,
    pub create_time: String,
    pub edit_time: String,
    pub tags: Vec<Tag>,
    pub category: Category,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrUpdatePost {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub content: String,
    pub summary: String,
    pub thumbnail: String,
    pub allow_comment: bool,
    pub published: bool,
    pub tags: Vec<i32>,
    pub category_id: i32,
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct PostYearArchive {
    pub year: String,
    pub months: Vec<PostMonthArchive>,
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct PostMonthArchive {
    pub month: String,
    pub num: i32,
}
