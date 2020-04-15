use super::{ Category, Tag };
#[derive(Serialize)]
pub struct Post {
    pub title: String,
    pub url: String,
    pub summary: String,
    pub thumbnail: String,
    pub reads: i32,
    //#[diesel(deserialize_as = "FormatedTime<HHMMDDHMTime>")]
    pub create_time: String,
    pub tags: Vec<Tag>,
    pub category: Category,
}