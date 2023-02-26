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
    let hostname = "0.0.0.0";

    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let finnHubToken = &args[2];
    
    print!("port: {}", port);
    print!("finnHubToken: {}", finnHubToken);
    
    env_logger::init();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");

    println!("Starting the server at {}:{}", hostname, port);

    domain::services::poll_service::poll(20);


    // let res = 
    //     reqwest::get("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY_ADJUSTED&symbol=NVDA&apikey=5580KYOVDAQFS7CJ")
    //         .await;
    
    // match res {
    //     Ok(res) => {
    //         println!("res: {:?}", res);
    //         match res.json::<domain::records::vantage_records::TimeSeriesDailyAdjusted>().await {
    //             Ok(text) => {
    //                 println!("Symbol: {:?}", text.meta_data.symbol);
    //                 // println!("{}", text.time_series_daily_adjusted.as_object().unwrap().keys().collect::<Vec<_>>()[0]);
    //                 for (key, value) in text.time_series_daily_adjusted.as_object().unwrap().iter() {
    //                     println!("\x1b[93mError\x1b[0m");
    //                     println!("{}: {}", key, value.as_object().unwrap().get("1. open").unwrap());
    //                 }

    //             },
    //             Err(err) => {
    //                 println!("err: {:?}", err);
    //             }
    //         }
    //     },
    //     Err(err) => {
    //         println!("err: {:?}", err);
    //     }
    // }




    
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
    .bind(format!("{}:{}", hostname, port))?
    .run()
    .await


}