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
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
    pub align: i32,
    pub data_type: i32,
    pub data: String,
}
