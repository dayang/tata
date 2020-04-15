use super::{ Category, Tag };
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