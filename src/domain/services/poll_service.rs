use std::time;
use std::thread;
use serde::Deserialize;
use reqwest::Error;


// Vantage API can't request more than 5 requests per minute

pub fn poll (sec:u64) -> () {
    thread::spawn(move || {
        loop {
            println!("time: {:?}", time::SystemTime::now());
            std::thread::sleep(time::Duration::from_secs(sec));
        }
    });
}