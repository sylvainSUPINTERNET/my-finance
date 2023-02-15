use actix_web::{web,get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware;
use std::{env, thread};
use std::io;
use std::time;
use log::{info, warn, debug, error};

mod domain;

async fn index() -> impl Responder {
    info!("Hello sunshine!");
    debug!("Hello sunshine!");
    print!("ok");
    HttpResponse::Ok().body("Hello sunshine!")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Starting the server at 0.0.0.0:9090");

    domain::PollerService::poller();    
    
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");

    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1")
                .route("/users", web::get().to(index)),
            )
            // // register HTTP requests handlers
            // .service(tweet::list)
            // .service(tweet::get)
            // .service(tweet::create)
            // .service(tweet::delete)
            // .service(like::list)
            // .service(like::plus_one)
            // .service(like::minus_one)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await


}