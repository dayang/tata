pub mod book;
pub mod comment;
pub mod friendlink;
pub mod pagination;
pub mod post;

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub err_msg: String,
}
