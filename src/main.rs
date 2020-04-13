extern crate bson;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;

use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use mongodb::Client;

pub mod db;
pub mod graphql_schema;
use crate::graphql_schema::{create_schema, Context, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok().expect("Problem at dotenv!");

    let mongodb_url = env::var(String::from("MONGODB_URL")).ok().expect("HEYOO");

    // Get a handle...
    let client = Client::with_uri_str(mongodb_url.as_str()).ok().expect("");

    // Put db connection handle to Schema Context.
    let schema_context = Context { db: client.clone() };

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(schema_context.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
