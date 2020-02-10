use crate::request::Request;
use reqwest::Client;
use std::{
    pin::Pin,
    task::{ Context, Poll }
};
use futures::FutureExt;
use async_std::{
    task,
    stream::Stream
};

#[derive(Clone, Debug)]
pub struct RequestList<'a> {
    sources: Vec<Request>,
    pub client: Option<&'a Client>
}

impl<'a> RequestList<'a> {
    fn new(sources: Vec<Request>) -> Self {
        RequestList {
            sources,
            client: None
        }
    }
}

impl<'a> Stream for RequestList<'a> {
    type Item = futures::future::BoxFuture<'a, String>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let req = self.sources.pop();
        if let Some(req) = req {
            let fut = req.fetch(self.client.unwrap()).boxed();
            return Poll::Ready(Some(fut))
        }
        Poll::Ready(None)
    }
}

impl From<Vec<Request>> for RequestList<'_> {
    fn from(src: Vec<Request>) -> Self {
        Self::new(src)
    }
}
