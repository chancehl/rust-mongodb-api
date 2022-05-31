use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

use crate::models::pet::Pet;

pub struct MongoDBRepo {
    collection: Collection<Pet>,
}

impl MongoDBRepo {
    pub fn init() -> Self {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
            Ok(value) => value.to_string(),
            Err(_) => format!("Error loading MongoDB URI from .env"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let collection: Collection<Pet> = db.collection("Pet");

        MongoDBRepo { collection }
    }

    pub fn create_pet(&self, new_pet: Pet) -> Result<InsertOneResult, Error> {
        let new_document = Pet {
            id: None,
            name: new_pet.name,
            breed: new_pet.breed,
        };

        let pet = self
            .collection
            .insert_one(new_document, None)
            .ok()
            .expect("Error creating pet");

        return Ok(pet);
    }

    pub fn get_pet(&self, id: &String) -> Result<Pet, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();

        let filter = doc! { "_id": object_id };

        let pet_detail = self
            .collection
            .find_one(filter, None)
            .ok()
            .expect("Error getting pet details");

        return Ok(pet_detail.unwrap());
    }

    pub fn update_pet(&self, id: &String, new_pet: Pet) -> Result<UpdateResult, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();

        let filter = doc! {"_id": object_id};

        let new_document = doc! {
            "$set":
                {
                    "id": new_pet.id,
                    "name": new_pet.name,
                    "breed": new_pet.breed,
                },
        };

        let updated_document = self
            .collection
            .update_one(filter, new_document, None)
            .ok()
            .expect("Error updating user");

        Ok(updated_document)
    }

    pub fn delete_pet(&self, id: &String) -> Result<DeleteResult, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();

        let filter = doc! {"_id": object_id};

        let pet_detail = self
            .collection
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting pet");

        Ok(pet_detail)
    }
}
