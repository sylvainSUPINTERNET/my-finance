use std::collections::HashMap;
use std::hash::Hash;
use std::time;
use std::thread;
use serde::Deserialize;
use reqwest::Error;

use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct RsiData {
    time: Vec<f64>,
    rsi: Vec<f64>,
    sLine: Vec<f64>, // smooth line
}

// Vantage API can't request more than 5 requests per minute


// cfpv1jhr01qmi6j4cgjgcfpv1jhr01qmi6j4cgk0
fn get_rsi (symbol: &str, period: &str, token:&str) -> Result<RsiData, reqwest::Error> {
    let client = Client::new();
    let resp = client.get("https://finnhub.io/api/v1/indicator")
        .query(&[("symbol", symbol), ("resolution", period), ("from", "0"), ("to", "9999999999"), ("indicator", "rsi"), ("timeperiod", "14"), ("seriestype", "close"), ("token", token)])
        .send()
        .unwrap()
        .json::<HashMap<String, Vec<f64>>>()
        .unwrap();

    let time = resp.get("t").map(|t| t.clone()).unwrap_or_else(Vec::new);
    let rsi = resp.get("rsi").map(|rsi| rsi.clone()).unwrap_or_else(Vec::new);
    let sLine = resp.get("sLine").map(|sl| sl.clone()).unwrap_or_else(Vec::new);

    return Ok(RsiData {
        time,
        rsi,
        sLine,
    });
}


pub fn poll (sec:u64) -> () {
    thread::spawn(move || {
        loop {
            // println!("time: {:?}", time::SystemTime::now());
            std::thread::sleep(time::Duration::from_secs(sec));
            // get_rsi();
        }
    });
}