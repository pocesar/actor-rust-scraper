use crate::input::Extract;
use crate::requestlist::RequestList;
use crate::scraper::{ Html, Selector };
use crate::serde_json::Value;
use futures::{
    future::Future,
    stream::{
        Stream,
        StreamExt,
        futures_unordered::FuturesUnordered
    }
};
use std::{
    collections::{ HashMap },
    time::{ Instant },
    pin::{ Pin },
    task::{ Context, Poll }
};

pub trait Extractor: Stream {
}

pub struct DataExtractor {}

impl DataExtractor {
    pub fn new() -> Self {
        DataExtractor {}
    }
}

impl Stream for DataExtractor {
    type Item = Option<Value>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Pending
    }
}

impl Extractor for DataExtractor {
}

// async fn extract_data_from_url_async(
//     req: &Request,
//     client: &reqwest::Client,
//     proxy_client: &reqwest::Client,
//     push_data_size: usize,
//     push_data_buffer: Arc<futures::lock::Mutex<Vec<serde_json::Value>>>,
//     force_cloud: bool,
//     debug_log: bool,
// ) {
//     let url = &req.url;
//     if debug_log {
//         println!("Started async extraction --- {}", url);
//     }

//     let now = Instant::now();
//     let response = request_text_async(url, &proxy_client).await;
//     let request_time = now.elapsed().as_millis();

//     // println!("Reqwest retuned");
//     match response {
//         Ok(html) => {
//             let now = Instant::now();
//             let dom = Html::parse_document(&html);
//             let parse_time = now.elapsed().as_millis();

//             let map: HashMap<String, Value> = HashMap::new();

//             let now = Instant::now();
//             let map = extract.iter().fold(map, move |mut acc, extr| {
//                 let selector_bind = &extr.selector;
//                 let selector = Selector::parse(&selector_bind).unwrap();
//                 let element = dom.select(&selector).next();
//                 let val = match element {
//                     Some(element) => {
//                         // println!("matched element");
//                         let extracted_value = match &extr.extract_type {
//                             ExtractType::Text => element
//                                 .text()
//                                 .fold(String::from(""), |acc, s| acc + s)
//                                 .trim()
//                                 .to_owned(),
//                             ExtractType::Attribute(at) => {
//                                 element.value().attr(&at).unwrap().to_owned()
//                             }
//                         };
//                         Some(extracted_value)
//                     }
//                     None => None,
//                 };
//                 acc.insert(
//                     extr.field_name.to_string(),
//                     val.map(Value::String).unwrap_or(Value::Null),
//                 );
//                 acc
//             });

//             let mapSize = map.len();

//             let value = serde_json::to_value(map).unwrap();
//             let extract_time = now.elapsed().as_millis();

//             let now = Instant::now();

//             {
//                 let mut locked_vec = push_data_buffer.lock().await;
//                 locked_vec.push(value);
//                 let vec_len = locked_vec.len();
//                 if debug_log {
//                     println!("Push  data buffer length:{}", vec_len);
//                 }
//                 if vec_len >= push_data_size {
//                     println!("Flushing data buffer --- length: {}", locked_vec.len());
//                     push_data_async(&locked_vec, &client, force_cloud).await;
//                     locked_vec.truncate(0);
//                     println!("Flushed data buffer --- length: {}", locked_vec.len());
//                 }
//             }

//             let push_time = now.elapsed().as_millis();

//             if debug_log {
//                 println!(
//                   "SUCCESS({}/{}) - {} - timings (in ms) - request: {}, parse: {}, extract: {}, push: {}",
//                   mapSize,
//                   extract.len(),
//                   req.url,
//                   request_time,
//                   parse_time,
//                   extract_time,
//                   push_time
//               );
//             }
//         }
//         Err(err) => {
//             println!(
//                 "FAILURE({} - timings (in ms) - request: {} --- {}",
//                 err, request_time, req.url,
//             );
//         }
//     }
// }

// fn extract_data_from_url(
//     req: &Request,
//     extract: &Vec<Extract>,
//     client: &reqwest::blocking::Client,
//     proxy_client: &reqwest::blocking::Client,
//     push_data_size: usize,
//     push_data_buffer: Arc<std::sync::Mutex<Vec<serde_json::Value>>>,
//     force_cloud: bool,
//     debug_log: bool,
// ) {
//     let url = &req.url;
//     if debug_log {
//         println!("Started sync extraction --- {}", url);
//     }

//     let now = Instant::now();
//     let html = request_text(&url, &proxy_client);
//     let request_time = now.elapsed().as_millis();

//     let now = Instant::now();
//     let dom = Html::parse_document(&html);
//     let parse_time = now.elapsed().as_millis();

//     let mut map: HashMap<String, Value> = HashMap::new();

//     let now = Instant::now();
//     extract.iter().for_each(|extr| {
//         let selector_bind = extr.selector.as_ref();
//         let selector = Selector::parse(&selector_bind).unwrap();
//         let element = dom.select(&selector).next();
//         let val = match element {
//             Some(element) => {
//                 // println!("matched element");
//                 let extracted_value = match &extr.extract_type {
//                     ExtractType::Text => element
//                         .text()
//                         .fold(String::from(""), |acc, s| acc + s)
//                         .trim()
//                         .to_owned(),
//                     ExtractType::Attribute(at) => element.value().attr(&at).unwrap().to_owned(),
//                 };
//                 Some(extracted_value)
//             }
//             None => None,
//         };
//         let insert_value = match val {
//             Some(string) => Value::String(string),
//             None => Value::Null,
//         };
//         map.insert(extr.field_name.to_string(), insert_value);
//     });

//     let mapSize = map.len();

//     let value = serde_json::to_value(map).unwrap();
//     let extractTime = now.elapsed().as_millis();

//     let now = Instant::now();
//     {
//         let mut locked_vec = push_data_buffer.lock().unwrap();
//         locked_vec.push(value);
//         let vec_len = locked_vec.len();
//         if debug_log {
//             println!("Push data buffer length:{}", vec_len);
//         }
//         if vec_len >= push_data_size {
//             println!("Flushing data buffer --- length: {}", locked_vec.len());
//             push_data(&locked_vec, &client, force_cloud);
//             locked_vec.truncate(0);
//             println!("Flushed data buffer --- length: {}", locked_vec.len());
//         }
//     }
//     let push_time = now.elapsed().as_millis();

//     if debug_log {
//         println!(
//             "SUCCESS({}/{}) - {} - timings (in ms) - request: {}, parse: {}, extract: {}, push: {}",
//             mapSize,
//             extract.len(),
//             &req.url,
//             request_time,
//             parse_time,
//             extractTime,
//             push_time
//         );
//     }
// }
