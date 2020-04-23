#[derive(Serialize, Deserialize)]
pub struct ApplyFriendLink {
    pub display_text: String,
    pub link: String,
    pub remark: String,
}
