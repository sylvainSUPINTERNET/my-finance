use std::time;
use std::thread;
use serde::Deserialize;
use reqwest::Error;

// // https://doc.rust-lang.org/rust-by-example/trait/dyn.html
// pub fn poller (start: str) -> Result<(), Box<dyn std::error::Error>> {
    
//     let mut interval = time::interval(time::Duration::from_secs(from_str::<int>(&*start)));

//     thread::spawn( move || {
//         loop {
//             interval.tick().await;
//             println!("Polling Vantage - time: {:?}", time::SystemTime::now());
//         }
//     });

// }

fn get_stock_EMA () -> Result<(), Error> {
    
    return Ok( () );
}

pub fn poller () -> () {
    thread::spawn(move || {
        loop {
            println!("time: {:?}", time::SystemTime::now());
            std::thread::sleep(time::Duration::from_secs(1));
        }
    });
} 