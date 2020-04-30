#[derive(Serialize, Deserialize)]
pub struct CatalogItem {
    pub id: i32,
    pub name: String,
    pub children: Vec<CatalogItem>,
}

#[derive(Serialize, Deserialize)]
pub struct CatalogItemUpdate {
    pub id: i32,
    pub name: String,
    pub parent_id: String,
}
