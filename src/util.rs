pub fn valid_year(year: i32) -> bool {
    year >= 1975 && year <= 2200
}

pub fn valid_month(month: i32) -> bool {
    month >= 1 && month <= 12
}