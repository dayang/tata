use serde::Serialize;

#[derive(Serialize)]
pub struct PaginationData<T: Serialize> {
    pub total_num: i64,
    pub total_pages: i64,
    pub per_page: i32,
    pub curr_page: i32,
    pub page_items: Vec<T>,
}

impl<T: Serialize> Default for PaginationData<T> {
    fn default() -> Self {
        PaginationData {
            total_num: 0,
            total_pages: 0,
            per_page: 0,
            curr_page: 0,
            page_items: vec![],
        }
    }
}
