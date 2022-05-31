// external modules
#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

// local modules
mod api;
mod models;
mod repository;

use api::pet::create_pet;
use repository::mongodb::MongoDBRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoDBRepo::init();

    rocket::build().manage(db).mount("/", routes![create_pet])
}
