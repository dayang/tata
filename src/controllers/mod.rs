
#[get("/")]
pub fn index() -> &'static str{
    "ta ta"
}

#[get("/<category>")]
pub fn category(category: String) -> String {
    category
}
