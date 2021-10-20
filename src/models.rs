use super::schema::elements;
use super::schema::pages;
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
#[table_name = "pages"]
pub struct Page {
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Queryable, Insertable, Serialize)]
#[table_name = "elements"]
pub struct Element {
    pub id: Option<i32>,
    pub page_id: i32,
    width: f32,
    height: f32,
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
    align: i32,
    data_type: i32,
    data: String,
}
