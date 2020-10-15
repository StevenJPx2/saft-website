#[macro_use]
extern crate diesel;

use std::env;

use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use diesel::prelude::*;

pub mod graphql;
pub mod models;
pub mod schema;

use crate::graphql::{Mutation, Query};

async fn index(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    req: Request,
) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    env_logger::init();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(Schema::new(Query, Mutation, EmptySubscription))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
