#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

mod models;
mod schema;

use diesel::prelude::*;
use models::*;
use rocket::serde::json::Json;
use rocket_cors::CorsOptions;
use schema::*;
use serde::Serialize;

#[database("database")]
pub struct DatabaseConnection(diesel::SqliteConnection);

#[get("/")]
async fn index(connection: DatabaseConnection) -> Json<Vec<Page>> {
    let result = connection
        .run(|connection| pages::table.load::<Page>(connection))
        .await;

    Json(match result {
        Ok(result) => result,
        Err(_) => vec![],
    })
}

#[post("/?<_title>")]
async fn add_page(_title: &str, connection: DatabaseConnection) -> &str {
    let _title = _title.to_string();

    let result = connection
        .run(|connection| {
            diesel::insert_into(schema::pages::table)
                .values(Page {
                    id: None,
                    title: _title,
                })
                .execute(connection)
        })
        .await;

    match result {
        Ok(_) => "Added page",
        Err(_) => "Error",
    }
}

#[derive(Serialize)]
struct PageDetails {
    pub id: Option<i32>,
    pub title: String,
    pub elements: Vec<Element>,
}

#[get("/?<_id>")]
async fn page(_id: i32, connection: DatabaseConnection) -> Json<PageDetails> {
    let result = connection
        .run(move |connection| {
            elements::table
                .filter(elements::page_id.eq(_id))
                .load::<Element>(connection)
        })
        .await;

    let elements = match result {
        Ok(result) => result,
        Err(_) => vec![],
    };

    let result = connection
        .run(move |connection| {
            pages::table
                .filter(pages::id.eq(_id))
                .load::<Page>(connection)
        })
        .await;

    let pages = match result {
        Ok(result) => result,
        Err(_) => vec![],
    };

    let page = pages.first();

    match page {
        Some(page) => Json(PageDetails {
            id: page.id,
            title: page.title.to_string(),
            elements,
        }),
        None => Json(PageDetails {
            id: Some(0),
            title: "Page not found".to_string(),
            elements: vec![],
        }),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DatabaseConnection::fairing())
        .attach(CorsOptions::default().to_cors().unwrap())
        .mount("/", routes![index, add_page])
        .mount("/page", routes![page])
}
