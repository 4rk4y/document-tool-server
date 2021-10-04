use super::schema::pages;
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
#[table_name = "pages"]
pub struct Page {
    pub id: Option<i32>,
    pub title: String,
}
