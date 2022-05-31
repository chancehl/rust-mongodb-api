use crate::{models::pet::Pet, repository::mongodb::MongoDBRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/pet", data = "<new_pet>")]
pub fn create_pet(
    db: &State<MongoDBRepo>,
    new_pet: Json<Pet>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Pet {
        id: None,
        name: new_pet.name.to_owned(),
        breed: new_pet.breed.to_owned(),
    };

    let pet_detail = db.create_pet(data);

    match pet_detail {
        Ok(pet) => Ok(Json(pet)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/pet/<path>")]
pub fn get_pet(db: &State<MongoDBRepo>, path: String) -> Result<Json<Pet>, Status> {
    let id = path;

    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let pet_detail = db.get_pet(&id);

    match pet_detail {
        Ok(pet) => Ok(Json(pet)),
        Err(_) => Err(Status::InternalServerError),
    }
}
