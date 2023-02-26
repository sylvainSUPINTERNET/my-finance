// part 2 : http server not required using WSS client
use std::error::Error;

use serde_json::json;
use tungstenite::{connect, Message};


mod domain;

const CONNECTION_STRING: &'static str = "wss://ws-feed.pro.coinbase.com";



fn main() -> Result<(), Box<dyn Error>> {
    
    // URL du canal de données Coinbase WebSocket

    // Paire de trading pour extraire les données
    let product_id = "BTC-USD";

    // Création d'un message WebSocket pour s'abonner au canal RSI
    let subscribe_message = json!({
        "type": "subscribe",
        "product_ids": [product_id],
        "channels": ["ticker"],
    });

    // Connexion au canal de données WebSocket
    let (mut socket, response) = connect(CONNECTION_STRING).unwrap();
    println!("Connected");

    // Envoi du message de souscription pour s'abonner au canal de données
    socket
        .write_message(Message::Text(subscribe_message.to_string()))
        .unwrap();

    loop {
        // Réception de messages du canal de données WebSocket
        let msg = socket.read_message().unwrap();

        if msg.is_text() {
            let data = msg.into_text().unwrap();

            // Traitement des données reçues
            let result: serde_json::Value = serde_json::from_str(&data).unwrap();

            // Extraction du RSI à partir des données reçues
            if let Some(rsi) = result["ticker"]["rsi"].as_f64() {
                println!("RSI: {}", rsi);
            }
        }
    }

    // let (mut socket, _) = connect("wss://ws.postman-echo.com/raw").unwrap();
    // println!("Connected to the server {:#?}", response.headers().get("Sec-WebSocket-Accept"));

    // let message = tungstenite::Message::Text("Hello, world!".into());
    // socket.write_message(message).unwrap();


    
    // println!("Connected");

    // loop {
    //     let msg = socket.read_message().unwrap();
    //     if msg.is_text() {
    //         let text = msg.to_text().unwrap();
    //         println!("Received message: {}", text);
    //     }
    // }
}






// --> PART 1 : 

// use actix_web::{web,get, App, HttpResponse, HttpServer, Responder};
// use actix_web::middleware;
// use std::{env, thread, result};
// use std::io;
// use std::time;
// use log::{info, warn, debug, error};

// mod domain;


// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello sunshine!")
// }

// #[actix_rt::main]
// async fn main() -> io::Result<()> {
//     let hostname = "0.0.0.0";

//     let args: Vec<String> = env::args().collect();
//     let port = &args[1];
//     let finn_hub_token = &args[2];

//     print!("port: {}", port);
//     print!("finnHubToken: {}", finn_hub_token);
    
//     env_logger::init();
//     env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");

//     println!("Starting the server at {}:{}", hostname, port);

//     domain::services::poll_service::poll(20);


//     // let res = 
//     //     reqwest::get("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY_ADJUSTED&symbol=NVDA&apikey=")
//     //         .await;
    
//     // match res {
//     //     Ok(res) => {
//     //         println!("res: {:?}", res);
//     //         match res.json::<domain::records::vantage_records::TimeSeriesDailyAdjusted>().await {
//     //             Ok(text) => {
//     //                 println!("Symbol: {:?}", text.meta_data.symbol);
//     //                 // println!("{}", text.time_series_daily_adjusted.as_object().unwrap().keys().collect::<Vec<_>>()[0]);
//     //                 for (key, value) in text.time_series_daily_adjusted.as_object().unwrap().iter() {
//     //                     println!("\x1b[93mError\x1b[0m");
//     //                     println!("{}: {}", key, value.as_object().unwrap().get("1. open").unwrap());
//     //                 }

//     //             },
//     //             Err(err) => {
//     //                 println!("err: {:?}", err);
//     //             }
//     //         }
//     //     },
//     //     Err(err) => {
//     //         println!("err: {:?}", err);
//     //     }
//     // }




    
//     HttpServer::new(|| {
//         App::new()
//             // enable logger - always register actix-web Logger middleware last
//             .wrap(middleware::Logger::default())
//             .service(
//                 web::scope("/api/v1")
//                 .route("/users", web::get().to(index)),
//             )
//             // // register HTTP requests handlers
//             // .service(tweet::list)
//             // .service(tweet::get)
//             // .service(tweet::create)
//             // .service(tweet::delete)
//             // .service(like::list)
//             // .service(like::plus_one)
//             // .service(like::minus_one)
//     })
//     .bind(format!("{}:{}", hostname, port))?
//     .run()
//     .await


// }