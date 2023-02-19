use actix_web::{web,get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware;
use std::{env, thread, result};
use std::io;
use std::time;
use log::{info, warn, debug, error};

mod domain;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello sunshine!")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Starting the server at 0.0.0.0:9090");

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");

    domain::services::poll_service::poll(5);


    let res = 
        reqwest::get("https://www.alphavantage.co/query?function=EMA&symbol=EWQ&interval=daily&time_period=10&series_type=open&apikey=5580KYOVDAQFS7CJ")
            .await;
    
    match res {
        Ok(res) => {
            println!("res: {:?}", res);
            match res.json::<domain::records::vantage_records::VantageEMA>().await {
                Ok(text) => {
                    println!("Symbol: {:?}", text.meta_data.symbol);
                },
                Err(err) => {
                    println!("err: {:?}", err);
                }
            }
        },
        Err(err) => {
            println!("err: {:?}", err);
        }
    }




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