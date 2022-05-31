// external modules
#[macro_use]
extern crate rocket;

// local modules
mod api;
mod models;
mod repository;

use api::pet::{create_pet, get_pet, update_pet};
use repository::mongodb::MongoDBRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoDBRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![create_pet])
        .mount("/", routes![get_pet])
        .mount("/", routes![update_pet])
}
