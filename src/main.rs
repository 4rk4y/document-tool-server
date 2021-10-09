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
use schema::pages::dsl::*;

#[database("database")]
pub struct DatabaseConnection(diesel::SqliteConnection);

#[get("/")]
async fn index(connection: DatabaseConnection) -> Json<Vec<Page>> {
    let result = connection.run(|c| pages.load::<Page>(c)).await;

    Json(match result {
        Ok(r) => r,
        Err(_) => vec![],
    })
}

#[post("/?<_title>")]
async fn add_page(_title: &str, connection: DatabaseConnection) -> &str {
    let _title = _title.to_string();

    let result = connection
        .run(|c| {
            diesel::insert_into(schema::pages::table)
                .values(Page {
                    id: None,
                    title: _title,
                })
                .execute(c)
        })
        .await;

    match result {
        Ok(_) => "Added page",
        Err(_) => "Error",
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DatabaseConnection::fairing())
        .attach(CorsOptions::default().to_cors().unwrap())
        .mount("/", routes![index, add_page])
}
