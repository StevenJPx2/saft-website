use actix_web::{middleware, App, HttpServer};

use async_graphql::{EmptySubscription, Schema};
use server::{
    data::PGQuery, db::get_pool, endpoints::graphql_endpoints, graphql::ContextData,
    graphql::Mutation, graphql::Query,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Playground: http://localhost:8000");

    let pool = get_pool();

    HttpServer::new(move || {
        let db = PGQuery { pool: pool.clone() };

        App::new()
            .data(pool.clone())
            .data(
                Schema::build(Query, Mutation, EmptySubscription)
                    .data(ContextData { db })
                    .finish(),
            )
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
