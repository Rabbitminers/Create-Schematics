use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct PersonFilter {
    pub name: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct SchematicFilter {
    pub title: Option<String>,
    pub author: Option<i32>,
    pub tags: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct CommentFilter {
    pub rating: Option<i32>,
    pub author: Option<i32>,
    pub date: Option<NaiveDateTime>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}
