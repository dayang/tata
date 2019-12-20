pub mod admin;
pub mod blog;

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub err_msg: String,
}