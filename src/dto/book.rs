#[derive(Serialize, Deserialize, Clone)]
pub struct CatalogItem {
    pub id: i32,
    pub url: String,
    #[serde(rename(serialize = "name", deserialize = "name"))]
    pub title: String,
    pub display_order: i32,
    pub parent_id: i32,
    pub children: Vec<CatalogItem>,
}

#[derive(Serialize, Deserialize)]
pub struct AddCatalogItem {
    pub url: String,
    pub title: String,
    pub book_id: i32,
    pub parent_id: i32,
    pub display_order: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MoveCatalogItem {
    pub id: i32,
    pub parent_id: i32,
    pub display_order: i32,
}
