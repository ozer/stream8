extern crate bson;
extern crate dotenv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use lazy_static::lazy_static;
use mongodb::Client;
use std::env;
use std::sync::Arc;

pub mod db;
pub mod graphql_schema;
use crate::graphql_schema::{create_schema, Context, Schema};

lazy_static! {
    static ref MONGODB_URL: String = env::var("MONGODB_URL").expect("NO MONGODB URL PROVIDED!");
}

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
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mongodb_url: String = format!("{}", *MONGODB_URL);

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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test};
    use juniper::http::GraphQLRequest;
    use mongodb;
    use std::env;

    fn setup_test() {
        dotenv().ok();
        if env::var_os("MONGODB_URL").is_none() {
            env::set_var("MONGODB_URL", "http://localhost:27017");
        }
    }

    fn get_mongo_handle() -> mongodb::Client {
        let mongodb_url: String = format!("{}", *MONGODB_URL);
        let client = mongodb::Client::with_uri_str(mongodb_url.as_str())
            .ok()
            .expect("Panic at getting connection to mongo!");
        return client;
    }

    fn get_graphql_request() -> GraphQLRequest {
        GraphQLRequest::new("query".to_string(), Some("name".to_string()), None)
    }

    #[actix_rt::test]
    async fn first_query() {
        setup_test();
        let client = get_mongo_handle().clone();
        let schema_context = Context { db: client.clone() };

        let schema = std::sync::Arc::new(create_schema());

        let mut app = test::init_service(
            App::new()
                .data(schema.clone())
                .data(schema_context.clone())
                .wrap(middleware::Logger::default())
                .service(web::resource("/graphql").route(web::post().to(graphql)))
                .service(web::resource("/graphiql").route(web::get().to(graphiql))),
        )
        .await;

        let req = test::TestRequest::post().uri("/graphql").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
