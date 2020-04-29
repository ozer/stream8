use crate::db;
use crate::graphql_schema;
use mongodb::Client;

pub fn create_streamer(client: &Client, firstname: String) -> Option<graphql_schema::Streamer> {
    println!("create_streamer");
    println!("{}", firstname);
    let result = db::create_streamer(
        client.clone(),
        db::NewStreamer {
            firstname: String::from(firstname),
        },
    )
    .ok()
    .expect("Problem at creating fake streamer!");
    println!("{:#?}", result);
    Some(graphql_schema::Streamer {
        id: result.inserted_id.to_string(),
        firstname: "ozer".to_string(),
    })
}

pub fn find_streamer_by_id(client: &Client, streamerId: bson::oid::ObjectId) -> Option<graphql_schema::Streamer> {
    Some(graphql_schema::Streamer {
        id: "123".to_string(),
        firstname: "ozer".to_string(),
    })
}
