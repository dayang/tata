use crate::schema::*;
use crate::sqltypes::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "book"]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub display_text: String,
    pub description: String,
    pub cover: String,
    pub published: bool,
    #[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub create_time: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "category"]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub display_text: String,
    pub remark: String,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, AsChangeset)]
#[table_name = "post"]
#[belongs_to(Category)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub content: String,
    pub summary: String,
    pub thumbnail: String,
    pub reads: i32,
    pub likes: i32,
    pub allow_comment: bool,
    pub published: bool,
    #[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub create_time: String,
    #[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub edit_time: String,
    pub category_id: i32,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, AsChangeset)]
// #[belongs_to(Post, foreign_key = "foreign_id")]
// #[belongs_to(Page, foreign_key = "foreign_id")]
#[table_name = "comment"]
pub struct Comment {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub content: String,
    #[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub comment_time: String,
    pub reply: String,
    #[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub reply_time: String,
    pub show: bool,
    pub foreign_id: i32,
    pub comment_type: i32,
    pub unread: bool,
    pub user_agent: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "dict"]
#[primary_key(d_key)]
pub struct Dict {
    pub d_key: String,
    pub d_value: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "friendlink"]
pub struct FriendLink {
    pub id: i32,
    pub display_text: String,
    pub link: String,
    pub show: bool,
    pub remark: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "logininfo"]
pub struct LoginInfo {
    pub id: i32,
    pub user_id: Option<i32>,
    pub username: String,
    #[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub login_time: String,
    pub is_success: bool,
    pub ip: String,
    pub mac: String,
    pub user_agent: String,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, AsChangeset)]
#[table_name = "page"]
#[belongs_to(Book)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub content: String,
    pub reads: i32,
    pub likes: i32,
    pub allow_comment: bool,
    pub published: bool,
    #[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub create_time: String,
    #[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub edit_time: String,
    pub parent_id: i32,
    pub book_id: i32,
    pub display_order: i32,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, AsChangeset)]
#[table_name = "posttag"]
#[belongs_to(Tag)]
#[belongs_to(Post)]
pub struct Posttag {
    pub id: i32,
    pub post_id: i32,
    pub tag_id: i32,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "tag"]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub display_text: String,
    pub remark: String,
    pub weight: i32,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub nick_name: String,
    pub description: String,
    pub password: String,
    pub avator: String,
    pub email: String,
    pub notify_comment: bool,
    pub notify_type: i32,
    pub notify_email: String,
    pub session_period: i32,
}
