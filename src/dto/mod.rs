pub mod admin;
// pub mod blog;
pub mod post;
pub mod comment;

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub err_msg: String,
}