use crate::{models::pet::Pet, repository::mongodb::MongoDBRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
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

#[put("/user/<path>", data = "<new_pet>")]
pub fn update_pet(
    db: &State<MongoDBRepo>,
    path: String,
    new_pet: Json<Pet>,
) -> Result<Json<Pet>, Status> {
    let id = path;

    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = Pet {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_pet.name.to_owned(),
        breed: new_pet.breed.to_owned(),
    };

    let update_result = db.update_pet(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_pet(&id);
                return match updated_user_info {
                    Ok(pet) => Ok(Json(pet)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
