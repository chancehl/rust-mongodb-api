use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
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
}
