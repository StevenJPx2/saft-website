use actix_web::{guard, web, HttpResponse};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{Request, Response};

use crate::graphql::{Mutation, Query};

async fn index(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    req: Request,
) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/").guard(guard::Post()).to(index))
        .service(web::resource("/").guard(guard::Get()).to(gql_playground));
}
