use mongodb::Client;
use crate::graphql_schema;
use crate::db;

pub fn create_streamer(client: &Client)->graphql_schema::Streamer {
  let _result = db::create_streamer(client.clone()).ok().expect("Problem at creating fake streamer!");
  println!("{:#?}", _result);
  graphql_schema::Streamer {
        id: String::from("123"),
        firstname: String::from("1233")
    }
}
