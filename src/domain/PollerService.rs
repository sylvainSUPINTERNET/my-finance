use std::{thread};

// https://doc.rust-lang.org/rust-by-example/trait/dyn.html
pub fn poller (start: str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut interval = time::interval(time::Duration::from_secs(from_str::<int>(&*start)));

    thread::spawn( move || {
        loop {
            interval.tick().await;
            println!("Polling Vantage - time: {:?}", time::SystemTime::now());
        }
    });

}