#[derive(Serialize, Deserialize)]
pub struct Dicts {
    pub index_title: String,
    pub meta_key_words: String,
    pub meta_description: String,
    pub site_info: String,
    pub copyright: String,
    pub post_page_num: String,
    pub comment_page_num: String,
    pub common_scripts: String,
    pub index_quote: String,
    pub about_page: String,
    pub avatar_url: String,
}
