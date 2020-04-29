use bson::{Bson, Document};
use mongodb::error::Error;
use mongodb::{options::InsertOneOptions, results::InsertOneResult, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Streamer {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: bson::oid::ObjectId,
    pub name: String,
    pub age: i32,
}

pub struct NewStreamer {
    pub firstname: String,
}

pub fn create_streamer(client: Client, streamer: NewStreamer) -> Result<InsertOneResult, Error> {
    let mut doc = Document::new();

    doc.insert("name".to_string(), Bson::String("ozer".to_string()));

    let insert_one_options = InsertOneOptions {
        bypass_document_validation: None,
        write_concern: None,
    };

    client
        .database("stream8")
        .collection("streamers")
        .insert_one(doc, insert_one_options)
}
