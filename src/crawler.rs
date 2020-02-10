use crate::requestlist::RequestList;
use crate::input::{ Input };
use crate::proxy::{ Proxy };
use crate::extractor::{ Extractor };
use std::{
    convert::{ TryFrom }
};
use crate::storage::Storage;
use futures::prelude::*;
use futures::{
    stream::{ StreamExt }
};

pub struct Crawler {
    input: Input
}

impl Crawler {
    pub fn new(
        input: Input
    ) -> Self {
        Crawler {
            input,
        }
    }

    pub async fn run<T: Extractor>(self, extractor: T, storage: Storage) {
        let settings = self.input;
        let client = reqwest::Client::builder().build().unwrap();

        let proxy_client = Proxy::try_from(settings.proxy_settings).map(|proxy| {
            let proxy_client = reqwest::Client::builder()
                .proxy(reqwest::Proxy::all(&proxy.base_url).unwrap().basic_auth(&proxy.username, &proxy.password))
                .build().unwrap();
            proxy_client
        }).unwrap_or(client);

        let mut req_list: RequestList = settings.urls.into();
        req_list.client = Some(&proxy_client);
        let concurrency: usize = 10;

        req_list.for_each_concurrent(concurrency, |req| async move {
            println!("len {}", req.await.len());
            ()
        }).await;
        // futures::future::join(
        //     reqList,
        //     future2: Fut2
        // ).await;
    }
}