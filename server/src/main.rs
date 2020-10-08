use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};

mod graphql;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Bruh")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .wrap(middleware::Logger::default())
            .wrap(actix_cors::Cors::default())
    })
    .bind("localhost:8080")?
    .run()
    .await
}
