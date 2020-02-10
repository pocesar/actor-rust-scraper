extern crate futures;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate async_std;

use async_std::prelude::*;

#[macro_use]
extern crate serde_derive;

pub trait Timestamp {
    fn timestamp(&self) -> u64;
}

impl Timestamp for SystemTime {
    fn timestamp(&self) -> u64 {
        self.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

fn timestamp() {
    print!("[{}]: ", SystemTime::now().timestamp());
}

#[macro_export]
macro_rules! debugln {
  () => { debugln!("(DEBUG)") };
  ($fmt:expr) => {
    if let Ok(_) = std::env::var("APIFY_LOG_LEVEL") {
        $crate::timestamp();
        println!($fmt);
    }
  };
  ($fmt:expr, $($arg:tt)*) => {
    if let Ok(_) = std::env::var("APIFY_LOG_LEVEL") {
        $crate::timestamp();
        println!($fmt, $($arg)*);
    }
  };
}

mod crawler;
mod extractor;
mod input;
mod proxy;
mod request;
mod requestlist;
mod storage;

use crate::crawler::Crawler;
use crate::extractor::DataExtractor;
use crate::input::{ Input };
use std::{
    env::args,
    time::SystemTime,
};
use storage::Storage;

// To not compile libraries on Apify, it is important to not commit Cargo.lock

#[tokio::main]
async fn main() {
    let storage = Storage::new(args().any(|arg| arg == "--force-cloud"));
    let input = storage.get_value::<Input>("INPUT").await.expect("Failed to read INPUT");

    debugln!("STATUS --- Loaded Input");

    //storage.start_send(SystemTime::now());

    Crawler::new(input).run(DataExtractor::new(), storage).await;
}
