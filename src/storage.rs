use futures::{
    Sink,
    SinkExt,
    stream::{ FuturesUnordered }
};
use std::{
    env,
    fmt::Debug,
    fs,
    pin::Pin,
    task::{Context, Poll},
};

use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::from_str;

pub async fn request_text(client: &reqwest::Client, url: &str) -> Result<String, reqwest::Error> {
    Ok(client.get(url).send().await?.text().await?)
}

pub struct Storage {
    client: reqwest::Client,
    force_cloud: bool,
    inner_key: u32,
    //queue: FuturesUnordered<Future>
}

impl Storage {
    pub fn new(force_cloud: bool) -> Self {
        debugln!("force cloud: {}", force_cloud);

        Storage {
            force_cloud,
            //queue: FuturesUnordered::new(),
            inner_key: 0,
            client: reqwest::Client::builder().build().unwrap(),
        }
    }

    fn create_indexed_key(&mut self) -> String {
        self.inner_key += 1;
        format!("{:08}", self.inner_key)
    }

    fn get_is_on_apify() -> bool {
        match env::var("APIFY_IS_AT_HOME") {
            Ok(ref x) if x == "1" => true,
            _ => false,
        }
    }

    pub async fn push_data<T: Sized + Serialize>(&mut self, data: &T) {
        let is_on_apify = Self::get_is_on_apify();
        let force_cloud = self.force_cloud;

        if is_on_apify || force_cloud {
            let json = serde_json::to_string(data).unwrap();
            let default_dataset = env::var("APIFY_DEFAULT_DATASET_ID").unwrap_or_else(move |_| {
                if force_cloud {
                    return "w7xbAHYhyoz3v8K8r".to_owned();
                }
                panic!("No APIFY_DEFAULT_DATASET_ID present or --force-cloud");
            });
            let token = env::var("APIFY_TOKEN").expect("APIFY_TOKEN must be provided");
            let url = format!(
                "https://api.apify.com/v2/datasets/{}/items?token={}",
                default_dataset, token
            );
            self.client
                .post(&url)
                .body(json)
                .header("Content-Type", "application/json")
                .send()
                .await
                .map_err(|err| {
                    debugln!("push_data error {}", err);
                })
                .ok();
        } else {
            let json = serde_json::to_string(&data).unwrap();
            let path = format!(
                "apify_storage/datasets/default/{}.json",
                self.create_indexed_key()
            );

            fs::write(path, json)
                .map_err(|err| {
                    debugln!("push_data error {}", err);
                })
                .ok();
        }
    }

    pub async fn get_value<'de, T>(&self, key: &str) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned + Debug,
    {
        let is_on_apify = Self::get_is_on_apify();

        debugln!("Is on Apify? -> {}", is_on_apify);

        let json = if is_on_apify {
            let default_kv = env::var("APIFY_DEFAULT_KEY_VALUE_STORE_ID").unwrap();
            debugln!("Default KV -> {}", default_kv);

            let url = format!(
                "https://api.apify.com/v2/key-value-stores/{}/records/{}",
                default_kv, key
            );

            request_text(&self.client, &url)
                .await
                .map_err(|err| {
                    debugln!("get_value failed: {}", err);
                })
                .unwrap_or_else(|_| "".to_owned())
        } else {
            fs::read_to_string(format!(
                "apify_storage/key_value_stores/default/{}.json",
                key
            ))
            .map_err(|err| {
                debugln!("get_value failed: {}", err);
            })
            .unwrap_or_else(|_| "".to_owned())
        };

        from_str(&json)
            .map(|input| {
                debugln!("Parsed input into: {:?}", input);
                input
            })
            .map_err(|error| {
                debugln!("Parsing failed with error: {}", error);
                error
            })
    }

    pub async fn set_value<T: Sized + Serialize>(&self, key: &str, value: &T) {
        let is_on_apify = Self::get_is_on_apify();
        let json = serde_json::to_string(&value).unwrap();

        if is_on_apify {
            let default_kv = env::var("APIFY_DEFAULT_KEY_VALUE_STORE_ID").unwrap();
            let token = env::var("APIFY_TOKEN").unwrap();
            let url = format!(
                "https://api.apify.com/v2/key-value-stores/{}/records/{}?token={}",
                default_kv, key, token
            );
            self.client
                .put(&url)
                .body(json)
                .header("Content-Type", "application/json")
                .send()
                .await
                .map_err(|err| {
                    debugln!("setValue failed {}", err);
                })
                .ok();
        } else {
            fs::write(
                format!("apify_storage/key_value_stores/default/{}.json", key),
                json,
            )
            .map_err(|err| {
                debugln!("setValue failed {}", err);
            })
            .ok();
        }
    }
}

impl<S: Serialize> Sink<S> for Storage {
    type Error = ();

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Pending
    }

    fn start_send(self: Pin<&mut Self>, item: S) -> Result<(), Self::Error> {
        //self.as_ref().queue.push(self.get_mut().push_data(&item));
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Pending
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Pending
    }
}